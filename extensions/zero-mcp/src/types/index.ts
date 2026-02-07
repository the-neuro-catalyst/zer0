export interface ProjectAnalysisResult {
    projectPath: string;
    packageName: string;
    packageVersion: string;
    totalFiles: number;
    fileTypes: {
        tsx: number;
        jsx: number;
        ts: number;
        js: number;
    };
    hasReact: boolean;
    hasTypeScript: boolean;
    hasVite: boolean;
    hasNext: boolean;
    hasTailwind: boolean;
    hasSupabase: boolean;
    timestamp: string;
}

export interface Dependency {
    name: string;
    version: string;
}

export interface DependencyCategories {
    react: Dependency[];
    ui: Dependency[];
    state: Dependency[];
    routing: Dependency[];
    styling: Dependency[];
    database: Dependency[];
    build: Dependency[];
    testing: Dependency[];
    utilities: Dependency[];
    other: Dependency[];
}

export interface DependenciesAnalysisResult {
    totalDependencies: number;
    totalDevDependencies: number;
    categories: DependencyCategories;
    frameworks: {
        hasReact: boolean;
        hasNext: boolean;
        hasVite: boolean;
        hasTailwind: boolean;
        hasSupabase: boolean;
        hasTypeScript: boolean;
    };
}

export interface ComponentDetail {
    path: string;
    name: string;
    size: number;
    hasDefaultExport: boolean;
    hasNamedExports: boolean;
    hasJSX: boolean;
    hasProps: boolean;
    hasState: boolean;
    hasEffects: boolean;
    isComponent: boolean;
}

export interface ComponentsAnalysisResult {
    totalComponents: number;
    analyzed: number;
    components: ComponentDetail[];
}

export interface RouteDetail {
    path: string;
    file: string;
    component: string;
    isProtected: boolean;
}

export interface RoutingAnalysisResult {
    routingFiles: string[];
    routes: RouteDetail[];
    totalRoutes: number;
    hasReactRouter: boolean;
}

export interface ClassUsageStats {
    class: string;
    count: number;
}

export interface FileClassUsage {
    file: string;
    uniqueClasses: number;
    totalClasses: number;
    topClasses: string[];
}

export interface TailwindPatterns {
    layout: ClassUsageStats[];
    colors: ClassUsageStats[];
    typography: ClassUsageStats[];
    responsive: ClassUsageStats[];
}

export interface TailwindUsageResult {
    totalFiles: number;
    analyzedFiles: number;
    totalUniqueClasses: number;
    mostUsedClasses: ClassUsageStats[];
    patterns: TailwindPatterns;
    fileUsage: FileClassUsage[];
}

export interface HookUsageStats {
    hook: string;
    count: number;
}

export interface FileHookUsage {
    file: string;
    builtInHooks: string[];
    customHooks: string[];
}

export interface HooksUsageResult {
    totalFiles: number;
    analyzedFiles: number;
    totalHooksUsage: number;
    mostUsedHooks: HookUsageStats[];
    customHooksFound: string[];
    fileUsage: FileHookUsage[];
}

export interface ApiCallDetail {
    type: "fetch" | "axios" | "supabase" | "api_endpoint";
    url?: string;
    method?: string;
    table?: string;
    endpoint?: string;
    file: string;
}

export interface ApiCallsAnalysisResult {
    totalApiCalls: number;
    apiTypes: Record<string, number>;
    methods: Record<string, number>;
    domains: Record<string, number>;
    calls: ApiCallDetail[];
    hasSupabase: boolean;
    hasAxios: boolean;
    hasFetch: boolean;
}

export interface SchemaTableDetail {
    name: string;
    file: string;
    definition: string; // truncated definition
}

export interface SchemaTypeDetail {
    name: string;
    file: string;
    type: "interface" | "type";
    definition: string; // truncated definition
}

export interface SchemaFunctionDetail {
    name: string;
    file: string;
}

export interface SchemaPolicyDetail {
    name: string;
    table: string;
    file: string;
}

export interface SchemaRelationshipDetail {
    referencedTable: string;
    file: string;
}

export interface SupabaseUsageDetail {
    table: string;
    file: string;
    operation: string; // e.g., "query"
}

export interface DatabaseSchema {
    tables: SchemaTableDetail[];
    types: SchemaTypeDetail[];
    functions: SchemaFunctionDetail[];
    policies: SchemaPolicyDetail[];
    relationships: SchemaRelationshipDetail[];
}

export interface DatabaseSchemaAnalysisResult {
    schemaFiles: string[];
    totalFiles: number;
    schema: DatabaseSchema;
    supabaseUsage: SupabaseUsageDetail[];
    statistics: {
        totalTables: number;
        totalTypes: number;
        totalFunctions: number;
        totalPolicies: number;
        totalRelationships: number;
        supabaseReferences: number;
    };
    hasSupabase: boolean;
    hasSQL: boolean;
    hasTypeScript: boolean;
}
