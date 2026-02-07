import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { z } from "zod";
import { join, resolve } from "node:path";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { existsSync } from "node:fs";

// Import new tools
import {
  analyzeProject,
  analyzeDependencies,
  getComponents,
  getRoutingStructure,
  getTailwindUsage,
  getHooksUsage,
  analyzeApiCalls,
  analyzeDatabaseSchema,
  getProjectResources,
  readProjectResource,
  getPromptTemplates,
  getPrompt,
} from "./tools/index.js";

const mcpServer = new McpServer({
  name: "zero-mcp",
  version: "0.2.0",
});

const execFileAsync = promisify(execFile);

// Configurable binary path with fallback
const getBinaryPath = async (): Promise<string> => {
  const envPath = process.env.ZERO_BINARY_PATH;
  if (envPath) {
    return envPath;
  }
  // The binaries are pre-built and located in 'src/zero'
  return join(process.cwd(), "src", "zero");
};

const binaryPath = getBinaryPath();

// Timeout configuration (in milliseconds)
const DEFAULT_TIMEOUT = 30000; // 30 seconds
const LONG_TIMEOUT = 120000; // 2 minutes for heavy operations

// Helper: Check if binary exists
const checkBinary = async (name: string): Promise<string> => {
  const path = join(await binaryPath, name);
  if (!existsSync(path)) {
    throw new Error(
      `Binary '${name}' not found at ${path}.\n` +
        `Did you run 'cargo build --release'?\n` +
        `You can set ZERO_BINARY_PATH environment variable to override the default location.`,
    );
  }
  return path;
};

// Helper: Execute with better error context
const execWithContext = async (
  binary: string,
  args: string[],
  options: {
    maxBuffer?: number;
    timeout?: number;
    context?: string;
  } = {},
): Promise<{ stdout: string; stderr: string }> => {
  const {
    maxBuffer = 1024 * 1024 * 50, // 50MB default
    timeout = DEFAULT_TIMEOUT,
    context = "operation",
  } = options;

  try {
    const result = await execFileAsync(binary, args, {
      maxBuffer,
      timeout,
    });
    return result;
  } catch (error: any) {
    // Timeout error
    if (error.killed && error.signal === "SIGTERM") {
      throw new Error(
        `${context} timed out after ${timeout}ms.\n` +
          `Consider processing smaller datasets or increasing timeout via ZERO_MCP_TIMEOUT env variable.`,
      );
    }

    // Binary not found
    if (error.code === "ENOENT") {
      throw new Error(
        `Failed to execute '${binary}': command not found.\n` +
          `Make sure the binary is installed and in your PATH.`,
      );
    }

    // Buffer overflow
    if (error.message.includes("maxBuffer")) {
      throw new Error(
        `${context} exceeded output buffer (${maxBuffer} bytes).\n` +
          `The data schema is too large. Try reducing --head parameter or processing a smaller file.`,
      );
    }

    // Generic error with context
    throw new Error(`${context} failed: ${error.message}`);
  }
};

// Keep process alive
const keepAlive = setInterval(() => {}, 1000);

// Project path from command line or current directory
const projectPathArg = process.argv.find((arg) =>
  arg.startsWith("--project-path="),
);
const resolvedPath = projectPathArg
  ? resolve(projectPathArg.split("=")[1])
  : resolve(process.cwd());

console.error(`Starting Zero MCP server for project: ${resolvedPath}`);

// Tool: Inspect Resource
mcpServer.registerTool(
  "inspect_resource",
  {
    description:
      "Deep analysis of a file or URL. Returns schema, metadata, and data samples. Supports Parquet, CSV, JSON, etc.",
    inputSchema: z.object({
      path: z.string().describe("File path to read"),
      head: z
        .number()
        .optional()
        .describe("Number of records to sample (default 10)"),
    }).shape,
  },
  async ({ path, head }) => {
    try {
      const readerPath = await checkBinary("zero-reader");
      const args = [
        "--file",
        path,
        "--head",
        (head || 10).toString(),
        "--format",
        "json",
      ];

      const { stdout, stderr } = await execWithContext(readerPath, args, {
        context: `Inspecting resource '${path}'`,
        timeout: LONG_TIMEOUT,
      });

      if (stderr && stderr.includes("ERROR")) {
        return {
          content: [
            {
              type: "text",
              text: `Failed to read file '${path}':\n${stderr}\n\nSupported formats: Parquet, CSV, JSON, JSONL, XML, YAML, XLSX`,
            },
          ],
          isError: true,
        };
      }

      return {
        content: [{ type: "text", text: stdout }],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Search Files
mcpServer.registerTool(
  "search_files",
  {
    description:
      "High-speed recursive search for a pattern within a directory using ripgrep. Supports regex.",
    inputSchema: z.object({
      pattern: z
        .string()
        .describe("The pattern or string to search for (regex supported)"),
      dirPath: z
        .string()
        .optional()
        .describe("Directory to search in (defaults to current directory)"),
      include: z
        .string()
        .optional()
        .describe('Glob pattern for files to include (e.g., "*.ts", "src/**")'),
    }).shape,
  },
  async ({ pattern, dirPath, include }) => {
    try {
      const args = [
        "--vimgrep",
        "--no-heading",
        "--line-number",
        "--color",
        "never",
        pattern,
      ];
      if (dirPath) args.push(dirPath);
      if (include) {
        args.push("-g");
        args.push(include);
      }

      const { stdout, stderr } = await execWithContext("rg", args, {
        context: `Searching for pattern '${pattern}'`,
      });

      if (stderr) {
        return {
          content: [{ type: "text", text: `Search Error: ${stderr}` }],
          isError: true,
        };
      }

      return {
        content: [{ type: "text", text: stdout || "No matches found." }],
      };
    } catch (error: any) {
      // rg returns exit code 1 if no matches are found
      if (error.code === 1) {
        return {
          content: [{ type: "text", text: "No matches found." }],
        };
      }
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: List Tree
mcpServer.registerTool(
  "list_tree",
  {
    description:
      "Provide a recursive tree view of the directory structure. Use this for a bird's-eye view of the project.",
    inputSchema: z.object({
      path: z.string().describe("Root directory path for the tree"),
      maxDepth: z
        .number()
        .optional()
        .default(3)
        .describe("Maximum depth to traverse (default: 3)"),
    }).shape,
  },
  async ({ path, maxDepth }) => {
    try {
      const args = [
        path,
        "-maxdepth",
        maxDepth.toString(),
        "-not",
        "-path",
        "*/.*",
      ];
      const { stdout, stderr } = await execWithContext("find", args, {
        context: `Listing directory tree for '${path}'`,
      });

      if (stderr) {
        return {
          content: [{ type: "text", text: `Tree Error: ${stderr}` }],
          isError: true,
        };
      }

      return {
        content: [{ type: "text", text: stdout || "Directory is empty." }],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Scan Patterns (with customizable patterns)
mcpServer.registerTool(
  "scan_patterns",
  {
    description:
      "Automated scan for sensitive patterns (PII, Secrets, API Keys) across the workspace. Can use preset patterns or custom ones.",
    inputSchema: z.object({
      dirPath: z
        .string()
        .optional()
        .describe("Directory to scan (defaults to current directory)"),
      customPatterns: z
        .array(z.string())
        .optional()
        .describe(
          "Custom regex patterns to search for (overrides presets if provided)",
        ),
      excludeEmails: z
        .boolean()
        .optional()
        .default(false)
        .describe("Exclude email pattern from scan to reduce false positives"),
    }).shape,
  },
  async ({ dirPath, customPatterns, excludeEmails }) => {
    try {
      const defaultPatterns = [
        "API_KEY",
        "SECRET",
        "PASSWORD",
        "TOKEN",
        "access_key",
        "private_key",
        "Bearer",
      ];

      if (!excludeEmails) {
        defaultPatterns.push("[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}");
      }

      const patterns = customPatterns || defaultPatterns;
      const patternString = patterns.join("|");
      const args = [
        "--vimgrep",
        "--no-heading",
        "--line-number",
        "--color",
        "never",
        patternString,
      ];
      if (dirPath) args.push(dirPath);

      const { stdout, stderr } = await execWithContext("rg", args, {
        context: "Scanning for sensitive patterns",
      });

      if (stderr) {
        return {
          content: [{ type: "text", text: `Scan Error: ${stderr}` }],
          isError: true,
        };
      }

      return {
        content: [
          {
            type: "text",
            text: stdout || "No sensitive patterns detected. System is clean.",
          },
        ],
      };
    } catch (error: any) {
      if (error.code === 1) {
        return {
          content: [
            {
              type: "text",
              text: "No sensitive patterns detected. System is clean.",
            },
          ],
        };
      }
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Query Database
mcpServer.registerTool(
  "query_database",
  {
    description:
      "Execute a read-only SQL query against a database (SQLite or PostgreSQL).",
    inputSchema: z.object({
      url: z
        .string()
        .describe(
          'Database URL (e.g., "sqlite://data.db" or "postgres://user:pass@host/db")',
        ),
      query: z.string().describe("Read-only SQL query to execute"),
    }).shape,
  },
  async ({ url, query }) => {
    try {
      const readerPath = await checkBinary("zero-reader");
      const args = ["db", "--url", url, "--query", query];

      const { stdout, stderr } = await execWithContext(readerPath, args, {
        context: `Executing query on database '${url}'`,
        timeout: LONG_TIMEOUT,
      });

      if (stderr && stderr.includes("ERROR")) {
        return {
          content: [
            {
              type: "text",
              text: `Database query failed:\n${stderr}\n\nCheck:\n- URL format is correct\n- Database is accessible\n- Query syntax is valid\n- Operation is read-only (INSERT/UPDATE/DELETE not allowed)`,
            },
          ],
          isError: true,
        };
      }

      return {
        content: [{ type: "text", text: stdout }],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Get Database Schema
mcpServer.registerTool(
  "get_database_schema",
  {
    description:
      "Extract and visualize the schema (tables and columns) of a database.",
    inputSchema: z.object({
      url: z.string().describe('Database URL (e.g., "sqlite://data.db")'),
    }).shape,
  },
  async ({ url }) => {
    try {
      const readerPath = await checkBinary("zero-reader");
      const args = ["db-schema", "--url", url];

      const { stdout, stderr } = await execWithContext(readerPath, args, {
        context: `Extracting schema from database '${url}'`,
      });

      if (stderr && stderr.includes("ERROR")) {
        return {
          content: [
            {
              type: "text",
              text: `Failed to extract schema:\n${stderr}\n\nCheck:\n- Database URL is correct\n- Database is accessible\n- You have read permissions`,
            },
          ],
          isError: true,
        };
      }

      return {
        content: [{ type: "text", text: stdout }],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Analyze Join Keys (with improved schema parsing)
mcpServer.registerTool(
  "analyze_join_keys",
  {
    description:
      "Analyze schemas of two data sources (File vs Database or File vs File) to suggest potential join keys.",
    inputSchema: z.object({
      sourceA: z
        .string()
        .describe("Path to first source (File path or DB URL)"),
      sourceB: z
        .string()
        .describe("Path to second source (File path or DB URL)"),
    }).shape,
  },
  async ({ sourceA, sourceB }) => {
    try {
      const readerPath = await checkBinary("zero-reader");

      // Helper to get schema with error handling
      const getSchema = async (src: string): Promise<any> => {
        const isDb = src.includes("://");
        const args = isDb
          ? ["db-schema", "--url", src]
          : ["--file", src, "--head", "1", "--format", "json"];

        const { stdout, stderr } = await execWithContext(readerPath, args, {
          context: `Reading schema from '${src}'`,
          timeout: LONG_TIMEOUT,
        });

        if (stderr && stderr.includes("ERROR")) {
          throw new Error(`Failed to read schema from ${src}: ${stderr}`);
        }

        try {
          return JSON.parse(stdout);
        } catch (e) {
          throw new Error(
            `Failed to parse schema JSON from ${src}. Output: ${stdout.substring(0, 200)}...`,
          );
        }
      };

      // Extract columns with robust fallbacks
      const getColumns = (data: any, source: string): string[] => {
        try {
          // Handle DB schema structure
          if (Array.isArray(data) && data[0]?.table) {
            return data.flatMap((t: any) => {
              // Try to extract columns from SQL definition if available
              if (t.columns && Array.isArray(t.columns)) {
                return t.columns;
              }
              return [`[Table: ${t.table}] (Manual inspection needed)`];
            });
          }

          // Handle File reader structure - try multiple possible formats
          const possibleFormats = ["Parquet", "Csv", "Json", "Jsonl"];
          for (const format of possibleFormats) {
            if (data[format] && Array.isArray(data[format])) {
              const firstRecord = data[format][0];
              if (firstRecord?.column_schemas) {
                return firstRecord.column_schemas.map((c: any) => c.name);
              }
            }
          }

          // Fallback: try to extract from any array structure
          const firstKey = Object.keys(data)[0];
          if (firstKey && Array.isArray(data[firstKey])) {
            const item = data[firstKey][0];
            if (item?.column_schemas) {
              return item.column_schemas.map((c: any) => c.name);
            }
          }

          throw new Error(
            `Unrecognized schema format from ${source}. Expected Parquet/CSV/JSON file or database schema.`,
          );
        } catch (e: any) {
          throw new Error(
            `Failed to extract columns from ${source}: ${e.message}`,
          );
        }
      };

      const [schemaA, schemaB] = await Promise.all([
        getSchema(sourceA),
        getSchema(sourceB),
      ]);

      const colsA = getColumns(schemaA, sourceA);
      const colsB = getColumns(schemaB, sourceB);

      // Find intersection (case-insensitive)
      const potentialKeys = colsA.filter((a: string) =>
        colsB.some((b: string) => a.toLowerCase() === b.toLowerCase()),
      );

      const result = {
        sourceA_columns: colsA,
        sourceB_columns: colsB,
        potential_join_keys:
          potentialKeys.length > 0
            ? potentialKeys
            : "No direct name matches found. Check structural compatibility manually.",
        analysis: {
          total_columns_a: colsA.length,
          total_columns_b: colsB.length,
          matching_columns: potentialKeys.length,
          recommendation:
            potentialKeys.length > 0
              ? `Use ${potentialKeys.join(" or ")} for joining.`
              : "Consider manual schema review or data transformation.",
        },
      };

      return {
        content: [
          {
            type: "text",
            text: JSON.stringify(result, null, 2),
          },
        ],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Ingest Data
mcpServer.registerTool(
  "ingest_data",
  {
    description:
      "Ingest data into a SQLite database with optional vector embeddings.",
    inputSchema: z.object({
      db_path: z.string().describe("Path to the SQLite database file"),
      collection_name: z.string().describe("Name of the collection (table)"),
      vector_size: z
        .number()
        .optional()
        .describe("Size of the vector for embeddings (e.g., 1536 for OpenAI)"),
      openai_api_key: z
        .string()
        .optional()
        .describe("OpenAI API key for embeddings"),
      embed_field: z
        .string()
        .optional()
        .describe("Field to use for embeddings"),
      path: z.string().describe("Path to the data to ingest"),
    }).shape,
  },
  async ({
    db_path,
    collection_name,
    vector_size,
    openai_api_key,
    embed_field,
    path,
  }) => {
    try {
      const ingestorPath = await checkBinary("ingestor");
      const args = [
        "sqlite",
        "--db-path",
        db_path,
        "--collection-name",
        collection_name,
        "--path",
        path,
      ];

      if (vector_size) {
        args.push("--vector-size", vector_size.toString());
      }
      if (openai_api_key) {
        args.push("--openai-api-key", openai_api_key);
      }
      if (embed_field) {
        args.push("--embed-field", embed_field);
      }

      const { stdout, stderr } = await execWithContext(ingestorPath, args, {
        context: `Ingesting data from '${path}' into '${db_path}'`,
        timeout: LONG_TIMEOUT * 2, // 4 minutes for ingestion
      });

      if (stderr && stderr.includes("ERROR")) {
        return {
          content: [
            {
              type: "text",
              text: `Data ingestion failed:\n${stderr}\n\nCheck:\n- Source file is readable\n- Database path is writable\n- OpenAI API key is valid (if using embeddings)\n- Embed field exists in source data`,
            },
          ],
          isError: true,
        };
      }

      return {
        content: [{ type: "text", text: stdout }],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// --- NEW TOOLS FROM DRAFT.TS REFACTORING ---

// Tool: Analyze Project
mcpServer.registerTool(
  "analyze_project",
  {
    description: "Analyze the Lovable project structure and configuration",
    inputSchema: z.object({}).shape,
  },
  async () => {
    try {
      return {
        content: [{ type: "text", text: await analyzeProject(resolvedPath) }],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Analyze Dependencies
mcpServer.registerTool(
  "analyze_dependencies",
  {
    description: "Analyze project dependencies and categorize them by type",
    inputSchema: z.object({}).shape,
  },
  async () => {
    try {
      return {
        content: [
          { type: "text", text: await analyzeDependencies(resolvedPath) },
        ],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Get Components
mcpServer.registerTool(
  "get_components",
  {
    description: "Get React components in the project with detailed analysis",
    inputSchema: z.object({}).shape,
  },
  async () => {
    try {
      return {
        content: [{ type: "text", text: await getComponents(resolvedPath) }],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Get Routing Structure
mcpServer.registerTool(
  "get_routing_structure",
  {
    description: "Analyze the application routing structure and routes",
    inputSchema: z.object({}).shape,
  },
  async () => {
    try {
      return {
        content: [
          { type: "text", text: await getRoutingStructure(resolvedPath) },
        ],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Get Tailwind Usage
mcpServer.registerTool(
  "get_tailwind_usage",
  {
    description: "Analyze Tailwind CSS class usage patterns and statistics",
    inputSchema: z.object({}).shape,
  },
  async () => {
    try {
      return {
        content: [{ type: "text", text: await getTailwindUsage(resolvedPath) }],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Get Hooks Usage
mcpServer.registerTool(
  "get_hooks_usage",
  {
    description: "Analyze React hooks usage patterns in the codebase",
    inputSchema: z.object({}).shape,
  },
  async () => {
    try {
      return {
        content: [{ type: "text", text: await getHooksUsage(resolvedPath) }],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Analyze API Calls
mcpServer.registerTool(
  "analyze_api_calls",
  {
    description: "Analyze external API calls and data fetching patterns",
    inputSchema: z.object({}).shape,
  },
  async () => {
    try {
      return {
        content: [{ type: "text", text: await analyzeApiCalls(resolvedPath) }],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Tool: Analyze Database Schema
mcpServer.registerTool(
  "analyze_database_schema",
  {
    description:
      "Analyze database schema, tables, types, functions, policies, and relationships",
    inputSchema: z.object({}).shape,
  },
  async () => {
    try {
      return {
        content: [
          { type: "text", text: await analyzeDatabaseSchema(resolvedPath) },
        ],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: error.message }],
        isError: true,
      };
    }
  },
);

// Register project resources
await (async () => {
  const projectResources = await getProjectResources(resolvedPath);
  for (const resource of projectResources) {
    mcpServer.registerResource(
      resource.name,
      resource.uri,
      {
        title: resource.name,
        description: resource.description,
        mimeType: resource.mimeType,
      },
      async (uri: URL) => {
        const content = await readProjectResource(uri.toString(), resolvedPath);
        return {
          contents: [
            {
              uri: uri.toString(),
              mimeType: resource.mimeType,
              text: content,
            },
          ],
        };
      },
    );
  }
})();

// === MCP SERVER REQUEST HANDLERS FOR RESOURCES AND PROMPTS ===

// Register MCP Prompts
await (async () => {
  try {
    const promptTemplates = await getPromptTemplates();
    for (const template of promptTemplates) {
      const argShape: Record<string, z.ZodTypeAny> = {};
      if (template.arguments) {
        for (const arg of template.arguments) {
          const zodString = z.string().describe(arg.description);
          argShape[arg.name] = arg.required ? zodString : zodString.optional();
        }
      }

      mcpServer.registerPrompt(
        template.name,
        {
          title: template.name
            .replace(/_/g, " ")
            .replace(/\b\w/g, (l: string) => l.toUpperCase()),
          description: template.description,
          argsSchema: argShape,
        },
        async (args) => {
          const result = await getPrompt(template.name, args);
          return {
            messages: result.messages as any,
          };
        },
      );
    }
    console.error("Successfully registered MCP prompts.");
  } catch (error) {
    console.error("Failed to register MCP prompts:", error);
  }
})();

// Handle process cleanup
process.on("SIGINT", () => {
  console.error("Received SIGINT, shutting down gracefully...");
  clearInterval(keepAlive);
  process.exit(0);
});

process.on("SIGTERM", () => {
  console.error("Received SIGTERM, shutting down gracefully...");
  clearInterval(keepAlive);
  process.exit(0);
});

// Start server
async function main() {
  try {
    console.error("Creating transport...");
    const transport = new StdioServerTransport();

    console.error("Connecting server...");
    await mcpServer.connect(transport);

    console.error(`Zero MCP server running for project: ${resolvedPath}`);
    console.error(
      "Server is ready with enhanced analysis tools, resources, and prompts...",
    );
  } catch (error) {
    console.error("Server error:", error);
    clearInterval(keepAlive);
    process.exit(1);
  }
}

main().catch((error) => {
  console.error("Main error:", error);
  clearInterval(keepAlive);
  process.exit(1);
});
