import fg from "fast-glob";
import { readFile } from "fs/promises";
import { basename, join } from "path";
import { ComponentDetail, ComponentsAnalysisResult, RouteDetail, RoutingAnalysisResult, ClassUsageStats, FileClassUsage, TailwindPatterns, TailwindUsageResult, HookUsageStats, FileHookUsage, HooksUsageResult } from '../types/index.js'; // Import new types

export async function getComponents(resolvedPath: string): Promise<string> {
  try {
    console.error("Getting components...");

    const componentFiles = await fg(["**/*.{tsx,jsx}"], {
      cwd: resolvedPath,
      ignore: ["**/node_modules/**", "**/dist/**", "**/build/**"],
      absolute: false,
    });

    console.error(`Found ${componentFiles.length} component files`);

    const components: ComponentDetail[] = [];
    for (const file of componentFiles.slice(0, 20)) {
      const fullPath = join(resolvedPath, file);
      const content = await readFile(fullPath, "utf-8");

      // Basic component analysis using regex
      const hasDefaultExport = /export\s+default/.test(content);
      const hasNamedExports =
        /export\s+(?:const|function|class|interface|type)/.test(content);
      const hasProps = /(?:props|Props)/.test(content);
      const hasState = /(?:useState|setState|state)/.test(content);
      const hasEffects = /useEffect/.test(content);

      // Extract component name from filename or default export
      const componentName = basename(file, ".tsx").replace(".jsx", "");

      components.push({
        path: file,
        name: componentName,
        size: content.length,
        hasDefaultExport,
        hasNamedExports,
        hasJSX: content.includes("<") && content.includes(">"),
        hasProps,
        hasState,
        hasEffects,
        isComponent: /export\s+(?:default\s+)?(?:function|const|class)/.test(
          content,
        ),
      });
    }

    const result: ComponentsAnalysisResult = {
      totalComponents: componentFiles.length,
      analyzed: components.length,
      components,
    };

    return JSON.stringify(result, null, 2);
  } catch (error: unknown) {
    if (error instanceof Error) {
      console.error("Error in getComponents:", error);
      return JSON.stringify({ error: error.message }, null, 2);
    }
    return JSON.stringify({ error: "An unknown error occurred" }, null, 2);
  }
}

export async function getRoutingStructure(resolvedPath: string): Promise<string> {
  try {
    console.error("Analyzing routing structure...");

    // Look for routing files
    const routingFiles = await fg(
      [
        "**/router*.{ts,tsx,js,jsx}",
        "**/routes*.{ts,tsx,js,jsx}",
        "**/App.{ts,tsx,js,jsx}",
        "**/main.{ts,tsx,js,jsx}",
        "**/index.{ts,tsx,js,jsx}",
      ],
      {
        cwd: resolvedPath,
        ignore: ["**/node_modules/**", "**/dist/**", "**/build/**"],
        absolute: false,
      },
    );

    const routes: RouteDetail[] = [];

    for (const file of routingFiles) {
      const fullPath = join(resolvedPath, file);
      const content = await readFile(fullPath, "utf-8");

      // Simple pattern matching for routes
      const routePatterns = [
        /path\s*[:=]\s*["']([^"']+)["']/g,
        /route\s*[:=]\s*["']([^"']+)["']/g,
        /<Route[^>]+path\s*=\s*["']([^"']+)["']/g,
        /\{\s*path\s*:\s*["']([^"']+)["']/g,
      ];

      for (const pattern of routePatterns) {
        let match;
        while ((match = pattern.exec(content)) !== null) {
          routes.push({
            path: match[1],
            file: file,
            component: basename(file, ".tsx").replace(".jsx", ""),
            isProtected: /protected|private|auth/i.test(content),
          });
        }
      }
    }

    const result: RoutingAnalysisResult = {
      routingFiles,
      routes: routes.slice(0, 20),
      totalRoutes: routes.length,
      hasReactRouter: routes.some((r) => r.path.includes("/")),
    };

    return JSON.stringify(result, null, 2);
  } catch (error: unknown) {
    if (error instanceof Error) {
      console.error("Error in getRoutingStructure:", error);
      return JSON.stringify({ error: error.message }, null, 2);
    }
    return JSON.stringify({ error: "An unknown error occurred" }, null, 2);
  }
}

export async function getTailwindUsage(resolvedPath: string): Promise<string> {
  try {
    console.error("Analyzing Tailwind usage...");

    const sourceFiles = await fg(["**/*.{tsx,jsx,ts,js}"], {
      cwd: resolvedPath,
      ignore: ["**/node_modules/**", "**/dist/**", "**/build/**"],
      absolute: false,
    });

    const classUsage = new Map<string, number>();
    const fileUsage: FileClassUsage[] = [];

    for (const file of sourceFiles.slice(0, 50)) {
      // Limit for performance
      const fullPath = join(resolvedPath, file);
      const content = await readFile(fullPath, "utf-8");

      // Extract className attributes and template literals
      const classMatches = [
        ...content.matchAll(/className\s*=\s*["']([^"']+)["']/g),
        ...content.matchAll(/className\s*=\s*{[^}]*["']([^"']+)["'][^}]*}/g),
        ...content.matchAll(/class\s*=\s*["']([^"']+)["']/g),
      ];

      const classNames: string[] = [];

      for (const match of classMatches) {
        const classes = match[1]?.split(/\s+/) || [];
        classNames.push(...classes);

        classes.forEach((cls) => {
          if (cls.trim() && cls.length > 0) {
            classUsage.set(cls, (classUsage.get(cls) || 0) + 1);
          }
        });
      }

      if (classNames.length > 0) {
        fileUsage.push({
          file,
          uniqueClasses: [...new Set(classNames)].length,
          totalClasses: classNames.length,
          topClasses: [...new Set(classNames)].slice(0, 10),
        });
      }
    }

    // Get most used classes
    const sortedClasses: ClassUsageStats[] = [...classUsage.entries()]
      .sort((a, b) => b[1] - a[1])
      .slice(0, 30)
      .map(([cls, count]) => ({ class: cls, count }));

    // Categorize common Tailwind patterns
    const patterns: TailwindPatterns = {
      layout: sortedClasses.filter((item) =>
        /^(flex|grid|block|inline|hidden|w-|h-|p-|m-|space-|gap-)/.test(item.class),
      ),
      colors: sortedClasses.filter((item) =>
        /^(bg-|text-|border-|from-|to-|via-)/.test(item.class),
      ),
      typography: sortedClasses.filter((item) =>
        /^(text-|font-|leading-|tracking-|uppercase|lowercase)/.test(item.class),
      ),
      responsive: sortedClasses.filter((item) =>
        /^(sm:|md:|lg:|xl:|2xl:)/.test(item.class),
      ),
    };

    const result: TailwindUsageResult = {
      totalFiles: sourceFiles.length,
      analyzedFiles: fileUsage.length,
      totalUniqueClasses: classUsage.size,
      mostUsedClasses: sortedClasses,
      patterns: {
        layout: patterns.layout
          .slice(0, 10),
        colors: patterns.colors
          .slice(0, 10),
        typography: patterns.typography
          .slice(0, 10),
        responsive: patterns.responsive
          .slice(0, 10),
      },
      fileUsage: fileUsage.slice(0, 10),
    };

    return JSON.stringify(result, null, 2);
  } catch (error: unknown) {
    if (error instanceof Error) {
      console.error("Error in getTailwindUsage:", error);
      return JSON.stringify({ error: error.message }, null, 2);
    }
    return JSON.stringify({ error: "An unknown error occurred" }, null, 2);
  }
}

export async function getHooksUsage(resolvedPath: string): Promise<string> {
  try {
    console.error("Analyzing React hooks usage...");

    const sourceFiles = await fg(["**/*.{tsx,jsx,ts,js}"], {
      cwd: resolvedPath,
      ignore: ["**/node_modules/**", "**/dist/**", "**/build/**"],
      absolute: false,
    });

    const hookUsage = new Map<string, number>();
    const customHooks: { hook: string; file: string }[] = [];
    const fileUsage: FileHookUsage[] = [];

    for (const file of sourceFiles.slice(0, 50)) {
      const fullPath = join(resolvedPath, file);
      const content = await readFile(fullPath, "utf-8");

      // Built-in hooks
      const builtInHooks = [
        "useState",
        "useEffect",
        "useContext",
        "useReducer",
        "useCallback",
        "useMemo",
        "useRef",
        "useLayoutEffect",
        "useDebugValue",
        "useId",
      ];

      const fileHooks: string[] = [];

      builtInHooks.forEach((hook) => {
        const regex = new RegExp(`\\b${hook}\\b`, "g");
        const matches = content.match(regex);
        if (matches) {
          fileHooks.push(hook);
          hookUsage.set(hook, (hookUsage.get(hook) || 0) + matches.length);
        }
      });

      // Custom hooks (functions starting with 'use')
      const customHookMatches =
        content.match(/\buse[A-Z][a-zA-Z0-9]*\b/g) || [];
      customHookMatches.forEach((hook) => {
        if (!builtInHooks.includes(hook)) {
          customHooks.push({ hook, file });
          hookUsage.set(hook, (hookUsage.get(hook) || 0) + 1);
        }
      });

      if (fileHooks.length > 0 || customHookMatches.length > 0) {
        fileUsage.push({
          file,
          builtInHooks: fileHooks,
          customHooks: customHookMatches.filter(
            (h) => !builtInHooks.includes(h),
          ),
        });
      }
      // Ensure fileHooks are unique when adding to fileUsage
      fileUsage[fileUsage.length - 1].builtInHooks = [...new Set(fileUsage[fileUsage.length - 1].builtInHooks)];
    }

    const sortedHooks: HookUsageStats[] = [...hookUsage.entries()]
      .sort((a, b) => b[1] - a[1])
      .map(([hook, count]) => ({ hook, count }));

    const result: HooksUsageResult = {
      totalFiles: sourceFiles.length,
      analyzedFiles: fileUsage.length,
      totalHooksUsage: [...hookUsage.values()].reduce((a, b) => a + b, 0),
      mostUsedHooks: sortedHooks,
      customHooksFound: [...new Set(customHooks.map((ch) => ch.hook))],
      fileUsage: fileUsage.slice(0, 15),
    };

    return JSON.stringify(result, null, 2);
  } catch (error: unknown) {
    if (error instanceof Error) {
      console.error("Error in getHooksUsage:", error);
      return JSON.stringify({ error: error.message }, null, 2);
    }
    return JSON.stringify({ error: "An unknown error occurred" }, null, 2);
  }
}
