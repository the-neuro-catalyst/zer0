import fg from "fast-glob";
import { readFile } from "fs/promises";
import { join } from "path";
import { getComponents, getRoutingStructure } from './web_ui_analysis.js'; // Import from web_ui_analysis

export async function getProjectResources(resolvedPath: string) {
  const resources = [
    {
      uri: "project://structure",
      name: "Project Structure",
      description: "Current project file tree and organization",
      mimeType: "application/json",
    },
    {
      uri: "project://package",
      name: "Package Information",
      description: "Package.json with dependencies and scripts",
      mimeType: "application/json",
    },
    {
      uri: "project://components",
      name: "Component Inventory",
      description: "React components with their relationships",
      mimeType: "application/json",
    },
    {
      uri: "project://routes",
      name: "Routing Configuration",
      description: "Application routing structure and parameters",
      mimeType: "application/json",
    },
  ];
  return resources;
}

export async function readProjectResource(uri: string, resolvedPath: string) {
  const resourceId = uri.replace("project://", "");

  switch (resourceId) {
    case "structure":
      const files = await fg(["**/*"], {
        cwd: resolvedPath,
        ignore: [
          "**/node_modules/**",
          "**/dist/**",
          "**/build/**",
          "**/.git/**",
        ],
        absolute: false,
      });
      return JSON.stringify(
        {
          totalFiles: files.length,
          files: files.slice(0, 100),
          directories: [...new Set(files.map((f) => f.split("/")[0]))].filter(
            (d) => d !== "",
          ),
        },
        null,
        2,
      );

    case "package":
      const packagePath = join(resolvedPath, "package.json");
      try {
        const content = await readFile(packagePath, "utf-8");
        return content;
      } catch {
        return JSON.stringify({ error: "No package.json found" }, null, 2);
      }

    case "components":
      return await getComponents(resolvedPath);
    case "routes":
      return await getRoutingStructure(resolvedPath);

    default:
      throw new Error(`Unknown resource: ${resourceId}`);
  }
}
