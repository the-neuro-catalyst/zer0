import fg from "fast-glob";
import { readFile } from "fs/promises";
import { join } from "path";
import { ProjectAnalysisResult, Dependency, DependencyCategories, DependenciesAnalysisResult } from '../types/index.js'; // Import new types

export async function analyzeProject(resolvedPath: string): Promise<string> {
  try {
    console.error("Analyzing project...");

    const packageJsonPath = join(resolvedPath, "package.json");
    let packageInfo: any = null; // Use 'any' for packageInfo as its structure is flexible

    try {
      const packageContent = await readFile(packageJsonPath, "utf-8");
      packageInfo = JSON.parse(packageContent);
    } catch {
      console.error("No package.json found");
    }

    const files = await fg(["**/*.{tsx,jsx,ts,js}"], {
      cwd: resolvedPath,
      ignore: ["**/node_modules/**", "**/dist/**", "**/build/**"],
      absolute: false,
    });

    console.error(`Found ${files.length} files`);

    const analysis: ProjectAnalysisResult = {
      projectPath: resolvedPath,
      packageName: packageInfo?.name || "Unknown",
      packageVersion: packageInfo?.version || "Unknown",
      totalFiles: files.length,
      fileTypes: {
        tsx: files.filter((f: string) => f.endsWith(".tsx")).length,
        jsx: files.filter((f: string) => f.endsWith(".jsx")).length,
        ts: files.filter((f: string) => f.endsWith(".ts")).length,
        js: files.filter((f: string) => f.endsWith(".js")).length,
      },
      hasReact: !!(
        packageInfo?.dependencies?.react || packageInfo?.devDependencies?.react
      ),
      hasTypeScript: !!(
        packageInfo?.dependencies?.typescript ||
        packageInfo?.devDependencies?.typescript
      ),
      hasVite: !!(
        packageInfo?.dependencies?.vite || packageInfo?.devDependencies?.vite
      ),
      hasNext: !!(
        packageInfo?.dependencies?.next || packageInfo?.devDependencies?.next
      ),
      hasTailwind: !!(
        packageInfo?.dependencies?.tailwindcss ||
        packageInfo?.devDependencies?.tailwindcss
      ),
      hasSupabase: !!(
        packageInfo?.dependencies?.["@supabase/supabase-js"] ||
        packageInfo?.devDependencies?.["@supabase/supabase-js"]
      ),
      timestamp: new Date().toISOString(),
    };

    return JSON.stringify(analysis, null, 2);
  } catch (error: unknown) {
    if (error instanceof Error) {
      console.error("Error in analyzeProject:", error);
      return JSON.stringify({ error: error.message }, null, 2);
    }
    return JSON.stringify({ error: "An unknown error occurred" }, null, 2);
  }
}

export async function analyzeDependencies(resolvedPath: string): Promise<string> {
  try {
    console.error("Analyzing dependencies...");

    const packageJsonPath = join(resolvedPath, "package.json");
    let packageInfo: any = null; // Use 'any' for packageInfo

    try {
      const packageContent = await readFile(packageJsonPath, "utf-8");
      packageInfo = JSON.parse(packageContent);
    } catch {
      return JSON.stringify({ error: "No package.json found" }, null, 2);
    }

    const dependencies: Record<string, string> = packageInfo.dependencies || {};
    const devDependencies: Record<string, string> = packageInfo.devDependencies || {};

    // Categorize dependencies
    const categories: DependencyCategories = {
      react: [],
      ui: [],
      state: [],
      routing: [],
      styling: [],
      database: [],
      build: [],
      testing: [],
      utilities: [],
      other: [],
    };

    const categorizePackage = (name: string, version: string) => {
      const pkg: Dependency = { name, version };
      if (name.includes("react") || name.includes("@types/react")) {
        categories.react.push(pkg);
      } else if (
        name.includes("ui") ||
        name.includes("component") ||
        name.includes("material") ||
        name.includes("ant") ||
        name.includes("chakra")
      ) {
        categories.ui.push(pkg);
      } else if (
        name.includes("redux") ||
        name.includes("zustand") ||
        name.includes("jotai") ||
        name.includes("recoil")
      ) {
        categories.state.push(pkg);
      } else if (
        name.includes("router") ||
        name.includes("navigation") ||
        name.includes("reach")
      ) {
        categories.routing.push(pkg);
      } else if (
        name.includes("styled") ||
        name.includes("emotion") ||
        name.includes("tailwind") ||
        name.includes("css") ||
        name.includes("sass")
      ) {
        categories.styling.push(pkg);
      } else if (
        name.includes("supabase") ||
        name.includes("prisma") ||
        name.includes("mongoose") ||
        name.includes("firebase")
      ) {
        categories.database.push(pkg);
      } else if (
        name.includes("vite") ||
        name.includes("webpack") ||
        name.includes("rollup") ||
        name.includes("babel") ||
        name.includes("esbuild")
      ) {
        categories.build.push(pkg);
      } else if (
        name.includes("test") ||
        name.includes("jest") ||
        name.includes("vitest") ||
        name.includes("cypress") ||
        name.includes("playwright")
      ) {
        categories.testing.push(pkg);
      } else if (
        name.includes("lodash") ||
        name.includes("axios") ||
        name.includes("dayjs") ||
        name.includes("uuid") ||
        name.includes("clsx")
      ) {
        categories.utilities.push(pkg);
      } else {
        categories.other.push(pkg);
      }
    };

    Object.entries(dependencies).forEach(([name, version]) =>
      categorizePackage(name, String(version)),
    );
    Object.entries(devDependencies).forEach(([name, version]) =>
      categorizePackage(name, String(version)),
    );

    const result: DependenciesAnalysisResult = { // Type the result
      totalDependencies: Object.keys(dependencies).length,
      totalDevDependencies: Object.keys(devDependencies).length,
      categories,
      frameworks: {
        hasReact: !!dependencies.react,
        hasNext: !!dependencies.next,
        hasVite: !!dependencies.vite || !!devDependencies.vite,
        hasTailwind:
          !!dependencies.tailwindcss || !!devDependencies.tailwindcss,
        hasSupabase: !!dependencies["@supabase/supabase-js"],
        hasTypeScript:
          !!devDependencies.typescript || !!dependencies.typescript,
      },
    };

    return JSON.stringify(result, null, 2);
  } catch (error: unknown) {
    if (error instanceof Error) {
      console.error("Error in analyzeDependencies:", error);
      return JSON.stringify({ error: error.message }, null, 2);
    }
    return JSON.stringify({ error: "An unknown error occurred" }, null, 2);
  }
}
