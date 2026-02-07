import fg from "fast-glob";
import { readFile } from "fs/promises";
import { join } from "path";
export async function analyzeApiCalls(resolvedPath) {
    try {
        console.error("Analyzing API calls...");
        const sourceFiles = await fg(["**/*.{tsx,jsx,ts,js}"], {
            cwd: resolvedPath,
            ignore: ["**/node_modules/**", "**/dist/**", "**/build/**"],
            absolute: false,
        });
        const apiCalls = [];
        const patterns = {
            fetch: /fetch\s*\(\s*["']([^"']+)["']/g,
            axios: /axios\s*\.\s*(get|post|put|delete|patch)\s*\(\s*["']([^"']+)["']/g,
            supabase: /supabase\s*\.\s*from\s*\(\s*["']([^"']+)["']\)/g,
            endpoints: /["']\/api\/([^"']+)["']/g,
        };
        for (const file of sourceFiles.slice(0, 50)) {
            const fullPath = join(resolvedPath, file);
            const content = await readFile(fullPath, "utf-8");
            // Fetch calls
            let match;
            while ((match = patterns.fetch.exec(content)) !== null) {
                apiCalls.push({
                    type: "fetch",
                    url: match[1],
                    file,
                    method: "GET", // Default, could be extracted from context
                });
            }
            // Axios calls
            while ((match = patterns.axios.exec(content)) !== null) {
                apiCalls.push({
                    type: "axios",
                    method: match[1].toUpperCase(),
                    url: match[2],
                    file,
                });
            }
            // Supabase calls
            while ((match = patterns.supabase.exec(content)) !== null) {
                apiCalls.push({
                    type: "supabase",
                    table: match[1],
                    file,
                });
            }
            // API endpoints
            while ((match = patterns.endpoints.exec(content)) !== null) {
                apiCalls.push({
                    type: "api_endpoint",
                    endpoint: match[1],
                    file,
                });
            }
        }
        // Analyze patterns
        const apiTypes = {};
        const methods = {};
        const domains = {};
        apiCalls.forEach((call) => {
            apiTypes[call.type] = (apiTypes[call.type] || 0) + 1;
            if (call.method) {
                methods[call.method] = (methods[call.method] || 0) + 1;
            }
            if (call.url) {
                try {
                    const url = new URL(call.url);
                    domains[url.hostname] = (domains[url.hostname] || 0) + 1;
                }
                catch {
                    // Not a full URL, probably relative
                    if (call.url.startsWith("/")) {
                        domains["relative"] = (domains["relative"] || 0) + 1;
                    }
                }
            }
        });
        const result = {
            totalApiCalls: apiCalls.length,
            apiTypes,
            methods,
            domains,
            calls: apiCalls.slice(0, 20),
            hasSupabase: Object.keys(apiTypes).includes("supabase"),
            hasAxios: Object.keys(apiTypes).includes("axios"),
            hasFetch: Object.keys(apiTypes).includes("fetch"),
        };
        return JSON.stringify(result, null, 2);
    }
    catch (error) {
        if (error instanceof Error) {
            console.error("Error in analyzeApiCalls:", error);
            return JSON.stringify({ error: error.message }, null, 2);
        }
        return JSON.stringify({ error: "An unknown error occurred" }, null, 2);
    }
}
export async function analyzeDatabaseSchema(resolvedPath) {
    try {
        console.error("Analyzing database schema...");
        const schemaFiles = await fg([
            "**/schema*.{sql,ts,js}",
            "**/database*.{sql,ts,js}",
            "**/migrations/**/*.{sql,ts,js}",
            "**/supabase/**/*.{sql,ts,js}",
            "**/types/**/*database*.{ts,js}",
            "**/types/**/*supabase*.{ts,js}",
        ], {
            cwd: resolvedPath,
            ignore: ["**/node_modules/**", "**/dist/**", "**/build/**"],
            absolute: false,
        });
        const schema = {
            tables: [],
            types: [],
            functions: [],
            policies: [],
            relationships: [],
        };
        const supabaseTypes = [];
        // const sqlStatements = []; // Removed: This variable was declared but never used in the original code.
        for (const file of schemaFiles) {
            const fullPath = join(resolvedPath, file);
            const content = await readFile(fullPath, "utf-8");
            // SQL table definitions
            const tableMatches = content.match(/CREATE\s+TABLE\s+(\w+)\s*\([^)]+\)/gi) || [];
            tableMatches.forEach((match) => {
                const tableName = match.match(/CREATE\s+TABLE\s+(\w+)/i)?.[1];
                if (tableName) {
                    schema.tables.push({
                        name: tableName,
                        file,
                        definition: match.slice(0, 200) + "...",
                    });
                }
            });
            // TypeScript interfaces for database
            const interfaceMatches = content.match(/interface\s+(\w*(?:Database|Table|Row|Insert|Update)\w*)\s*\{[^}]+\}/gi) || [];
            interfaceMatches.forEach((match) => {
                const interfaceName = match.match(/interface\s+(\w+)/i)?.[1];
                if (interfaceName) {
                    schema.types.push({
                        name: interfaceName,
                        file,
                        type: "interface",
                        definition: match.slice(0, 200) + "...",
                    });
                }
            });
            // Type aliases for database
            const typeMatches = content.match(/type\s+(\w*(?:Database|Table|Row|Insert|Update)\w*)\s*=[^;]+/gi) || [];
            typeMatches.forEach((match) => {
                const typeName = match.match(/type\s+(\w+)/i)?.[1];
                if (typeName) {
                    schema.types.push({
                        name: typeName,
                        file,
                        type: "type",
                        definition: match.slice(0, 200) + "...",
                    });
                }
            });
            // RLS Policies
            const policyMatches = content.match(/CREATE\s+POLICY\s+(\w+)\s+ON\s+(\w+)/gi) || [];
            policyMatches.forEach((match) => {
                const policyMatch = match.match(/CREATE\s+POLICY\s+(\w+)\s+ON\s+(\w+)/i);
                if (policyMatch) {
                    schema.policies.push({
                        name: policyMatch[1],
                        table: policyMatch[2],
                        file,
                    });
                }
            });
            // Functions/Procedures
            const functionMatches = content.match(/CREATE\s+(?:OR\s+REPLACE\s+)?FUNCTION\s+(\w+)/gi) || [];
            functionMatches.forEach((match) => {
                const funcName = match.match(/CREATE\s+(?:OR\s+REPLACE\s+)?FUNCTION\s+(\w+)/i)?.[1];
                if (funcName) {
                    schema.functions.push({
                        name: funcName,
                        file,
                    });
                }
            });
            // Foreign key relationships (simplified)
            const foreignKeyMatches = content.match(/REFERENCES\s+(\w+)\s*\(/gi) || [];
            foreignKeyMatches.forEach((match) => {
                const referencedTable = match.match(/REFERENCES\s+(\w+)/i)?.[1];
                if (referencedTable) {
                    schema.relationships.push({
                        referencedTable,
                        file,
                    });
                }
            });
            // Supabase specific patterns
            if (content.includes("supabase") || content.includes("Database")) {
                // Extract Supabase table references
                const supabaseTableRefs = content.match(/from\s*\(\s*["'](\w+)["']\s*\)/gi) || [];
                supabaseTableRefs.forEach((match) => {
                    const tableName = match.match(/["'](\w+)["']/)?.[1];
                    if (tableName) {
                        supabaseTypes.push({
                            table: tableName,
                            file,
                            operation: "query",
                        });
                    }
                });
            }
        }
        // Look for Supabase client usage in hooks and components
        const codeFiles = await fg(["**/*.{tsx,jsx,ts,js}"], {
            cwd: resolvedPath,
            ignore: ["**/node_modules/**", "**/dist/**", "**/build/**"],
            absolute: false,
        });
        const supabaseUsage = [];
        for (const file of codeFiles.slice(0, 30)) {
            const fullPath = join(resolvedPath, file);
            const content = await readFile(fullPath, "utf-8");
            if (content.includes("supabase")) {
                const tableRefs = content.match(/\.from\s*\(\s*["'](\w+)["']\s*\)/g) || [];
                tableRefs.forEach((match) => {
                    const tableName = match.match(/["'](\w+)["']/)?.[1];
                    if (tableName) {
                        supabaseUsage.push({
                            table: tableName,
                            file,
                            operation: "query",
                        });
                    }
                });
            }
        }
        const result = {
            schemaFiles,
            totalFiles: schemaFiles.length,
            schema: {
                tables: schema.tables.slice(0, 20),
                types: schema.types.slice(0, 20),
                functions: schema.functions.slice(0, 10),
                policies: schema.policies.slice(0, 10),
                relationships: schema.relationships.slice(0, 10),
            },
            supabaseUsage: supabaseUsage.slice(0, 15),
            statistics: {
                totalTables: schema.tables.length,
                totalTypes: schema.types.length,
                totalFunctions: schema.functions.length,
                totalPolicies: schema.policies.length,
                totalRelationships: schema.relationships.length,
                supabaseReferences: supabaseUsage.length,
            },
            hasSupabase: supabaseUsage.length > 0 || supabaseTypes.length > 0,
            hasSQL: schemaFiles.some((f) => f.endsWith(".sql")),
            hasTypeScript: schemaFiles.some((f) => f.endsWith(".ts")),
        };
        return JSON.stringify(result, null, 2);
    }
    catch (error) {
        if (error instanceof Error) {
            console.error("Error in analyzeDatabaseSchema:", error);
            return JSON.stringify({ error: error.message }, null, 2);
        }
        return JSON.stringify({ error: "An unknown error occurred" }, null, 2);
    }
}
