export async function getPromptTemplates() {
  return [
    {
      name: "code_review",
      description: "Review React component code and suggest improvements",
      arguments: [
        {
          name: "component_path",
          description: "Path to the component file",
          required: true,
        },
      ],
    },
    {
      name: "refactor_suggest",
      description: "Suggest refactoring improvements for better code structure",
      arguments: [
        {
          name: "file_path",
          description: "Path to the file to refactor",
          required: true,
        },
      ],
    },
    {
      name: "performance_audit",
      description: "Analyze component performance and suggest optimizations",
      arguments: [
        {
          name: "component_name",
          description: "Name of the component to audit",
          required: true,
        },
      ],
    },
  ];
}

export async function getPrompt(name: string, args: any) {
  switch (name) {
    case "code_review":
      return {
        messages: [
          {
            role: "user",
            content: {
              type: "text",
              text: `Please review the React component at ${args.component_path} and provide suggestions for improvements. Focus on:
1. Code structure and organization
2. Performance optimizations
3. Accessibility compliance
4. Best practices adherence
5. Potential bugs or issues`,
            },
          },
        ],
      };

    case "refactor_suggest":
      return {
        messages: [
          {
            role: "user",
            content: {
              type: "text",
              text: `Analyze the file at ${args.file_path} and suggest refactoring improvements. Consider:
1. Code duplication elimination
2. Function extraction opportunities
3. Better naming conventions
4. Improved error handling
5. Enhanced readability`,
            },
          },
        ],
      };

    case "performance_audit":
      return {
        messages: [
          {
            role: "user",
            content: {
              type: "text",
              text: `Perform a performance audit on the component "${args.component_name}". Check for:
1. Unnecessary re-renders
2. Memory leaks
3. Bundle size impact
4. Load time optimizations
5. Runtime performance bottlenecks`,
            },
          },
        ],
      };

    default:
      throw new Error(`Unknown prompt: ${name}`);
  }
}
