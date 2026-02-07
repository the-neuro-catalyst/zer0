# Product Requirements Document: Gemini CLI Extensibility Features (Custom Commands, Policy Engine, Agent Skills)

## 1. Introduction

This document outlines the product requirements for the Gemini CLI's core extensibility features: Custom Commands, the Policy Engine, and Agent Skills. These features empower users and the agent to customize, control, and extend the CLI's capabilities by defining personal shortcuts, enforcing security policies, and encapsulating complex workflows.

The `@extensions/zero-mcp-gemini` component, while not directly implementing these core features, provides a rich set of specialized tools, resources, and prompts that leverage and integrate with the extensibility mechanisms described herein. This PRD aims to clarify how these core features work and how the `zero-mcp-gemini` extension can best align with and benefit from them.

## 2. Goals

* **Enhance User Productivity:** Provide users with mechanisms to automate repetitive tasks and create personalized workflows.
* **Ensure Security & Control:** Implement robust controls over tool execution, protecting the user's environment and data.
* **Facilitate Advanced Automation:** Enable the agent to perform complex, multi-step tasks efficiently and reliably.
* **Promote Shareable Expertise:** Allow users and teams to package and share specialized knowledge and workflows.
* **Leverage Extension Capabilities:** Provide clear pathways for extensions like `zero-mcp-gemini` to expose their functionality through these extensibility points.

## 3. Audience

* **Gemini CLI Users:** Individuals who want to customize their CLI experience, create personal shortcuts, and understand security controls.
* **Extension Developers:** Developers building Gemini CLI extensions (e.g., `zero-mcp-gemini`) who need to understand how their tools, resources, and prompts are consumed and governed.
* **Agent Developers:** Engineers designing and implementing the Gemini agent's behavior, particularly concerning tool usage, workflow automation, and skill activation.

## 4. Feature Descriptions

### 4.1 Custom Commands

**Overview:** Custom Commands allow users to define lightweight, parameterized shortcuts for common prompts and tool invocations within the Gemini CLI. They streamline interactions by pre-packaging instructions and dynamic data inputs.

**Functional Requirements:**

* **FR1.1 - Definition Format:** Users SHALL define custom commands using the TOML file format (`.toml`).
* **FR1.2 - Prompt Specification:** Each command MUST specify a `prompt` string (single or multi-line) that will be sent to the Gemini model upon execution.
* **FR1.3 - Optional Description:** Commands MAY include an optional `description` for display in the `/help` menu.
* **FR1.4 - Discovery & Precedence:**
  * CLI MUST discover commands from `~/.gemini/commands/` (global) and `<project-root>/.gemini/commands/` (project-specific).
  * Project-specific commands MUST override global commands with the same name.
* **FR1.5 - Naming & Namespacing:**
  * Command names MUST be derived from their file paths relative to the `commands/` directory.
  * Subdirectories MUST create namespaced commands (e.g., `git/commit.toml` -> `/git:commit`).
* **FR1.6 - Argument Handling (`{{args}}`):**
  * The `prompt` MAY contain a `{{args}}` placeholder.
  * If present outside `!{...}` blocks, `{{args}}` MUST be replaced with the user's raw input.
  * If present inside `!{...}` blocks, `{{args}}` MUST be shell-escaped before injection into the shell command.
* **FR1.7 - Default Argument Handling:** If `{{args}}` is NOT present, the CLI MUST append the full command and its arguments (separated by two newlines) to the end of the `prompt`.
* **FR1.8 - Shell Command Execution (`!{...}`):**
  * The `prompt` MAY contain `!{...}` blocks for executing shell commands.
  * The output of these shell commands MUST be injected into the `prompt`.
  * The CLI MUST prompt the user for confirmation before executing any shell command, displaying the exact command(s) to be run.
  * Error messages (stderr) and exit codes from failed shell commands MUST be injected into the `prompt` to inform the model.
* **FR1.9 - File Content Injection (`@{...}`):**
  * The `prompt` MAY contain `@{...}` blocks for embedding file content or directory listings.
  * For text files, content MUST be injected directly.
  * For supported multimodal files (images, PDFs, audio, video), content MUST be encoded and injected as multimodal input. Other binary files SHOULD be skipped gracefully.
  * For directory paths, a listing of files within the directory (respecting `.gitignore` and `.geminiignore`) MUST be injected.
  * File content injection MUST occur before shell command execution and argument substitution.
* **FR1.10 - Workspace Awareness:** Paths specified in `@{...}` MUST be resolved relative to the current workspace or as absolute paths within the workspace.

**Technical Considerations (Integration with `zero-mcp-gemini`):**

* The `zero-mcp-gemini` extension registers various tools (e.g., `analyze_project`, `get_components`, `analyze_api_calls`, `query_database`). Custom Commands provide an intuitive way for users to create shortcuts that invoke these specific tools, potentially pre-filling arguments or injecting context from files.
* Example: A custom command `/project-summary` could use `!{zero-mcp-gemini__analyze_project}` or inject `@package.json` into a prompt that then asks for a summary.

### 4.2 Policy Engine

**Overview:** The Policy Engine provides administrators and users with fine-grained control over which tools the Gemini agent can execute, under what conditions, and whether user confirmation is required. It prioritizes security and predictable agent behavior.

**Functional Requirements:**

* **FR2.1 - Rule Definition:** Policies MUST be defined in TOML files (`.toml`) located in `~/.gemini/policies/`.
* **FR2.2 - Rule Structure:** Each rule MUST specify:
  * `toolName`: The name(s) of the tool(s) to which the rule applies (supports arrays and MCP wildcards `mcpName__*`).
  * `decision`: One of `allow`, `deny`, or `ask_user`.
  * `priority`: An integer from 0-999.
* **FR2.3 - Rule Conditions:** Rules MAY specify additional conditions:
  * `mcpName`: To target tools from a specific Model-Context-Protocol (MCP) server.
  * `argsPattern`: A regular expression to match against the JSON representation of the tool's arguments.
  * `commandPrefix`: (Syntactic sugar for `run_shell_command`) A string or array of strings that the shell command must start with.
  * `commandRegex`: (Syntactic sugar for `run_shell_command`) A regular expression to match against the entire shell command string.
  * `modes`: An array of approval modes (e.g., `yolo`, `autoEdit`) for which the rule is active.
* **FR2.4 - Priority Resolution:** When multiple rules match a tool call, the rule with the highest `final_priority` MUST determine the outcome.
  * Final priority calculation: `tier_base + (toml_priority / 1000)`.
  * Tiers (and their base values): Default (1), User (2), Admin (3). Admin policies MUST always override User and Default policies. User policies MUST always override Default policies.
* **FR2.5 - Non-Interactive Mode:** In non-interactive CLI sessions, `ask_user` decisions MUST be treated as `deny`.
* **FR2.6 - Default Policies:** The CLI MUST ship with default policies that:
  * `allow` read-only tools (e.g., `read_file`, `glob`).
  * `ask_user` for `delegate_to_agent` tool calls.
  * `ask_user` for write tools (e.g., `write_file`, `run_shell_command`).
  * Include a high-priority rule to `allow` all tools in `yolo` mode.
  * Include rules to `allow` certain write operations without prompt in `autoEdit` mode.

**Technical Considerations (Integration with `zero-mcp-gemini`):**

* The `zero-mcp-gemini` extension registers numerous tools (e.g., `inspect_resource`, `scan_patterns`, `ingest_data`, `analyze_database_schema`). The Policy Engine is crucial for governing access to and execution of these tools. For example, an administrator could define a policy to:
  * `deny` the `ingest_data` tool if the `db_path` argument matches a critical production database.
  * `ask_user` for confirmation whenever `scan_patterns` is run with `customPatterns` to prevent malicious regex.
* The `mcpName` condition in policy rules is directly relevant, allowing policies to target tools specifically from `zero-mcp-gemini` (e.g., `zero-mcp-gemini__inspect_resource`).

### 4.3 Agent Skills

**Overview:** Agent Skills provide a powerful mechanism to extend the Gemini CLI with specialized, on-demand expertise. They encapsulate instructions, procedural workflows, and associated resources into discoverable, self-contained directories, enabling the agent to perform complex tasks efficiently and consistently without cluttering its continuous context.

**Functional Requirements:**

* **FR3.1 - Skill Structure:** A skill MUST be a directory containing a `SKILL.md` file at its root.
* **FR3.2 - `SKILL.md` Format:** `SKILL.md` MUST use YAML frontmatter for metadata (`name`, `description`) and Markdown for detailed instructions.
  * `name`: A unique, alphanumeric identifier (lowercase, dashes).
  * `description`: A concise explanation of the skill's expertise, used by the agent to determine relevance.
  * Body: Expert procedural guidance for the agent's behavior.
* **FR3.3 - Resource Bundling (Conventions):** Skills SHOULD support optional subdirectories for resources:
  * `scripts/`: Executable scripts (bash, Python, Node.js).
  * `references/`: Static documentation, schemas, or example data.
  * `assets/`: Code templates, boilerplate, or binary resources.
* **FR3.4 - Skill Discovery Tiers & Precedence:** The CLI MUST discover skills from:
  * Workspace: `.gemini/skills/` (highest precedence)
  * User: `~/.gemini/skills/`
  * Extension: Skills bundled within installed extensions (lowest precedence)
  * Higher-precedence skills with the same name MUST override lower-precedence ones.
* **FR3.5 - Skill Management (Interactive):** Users MUST be able to manage skills via slash commands:
  * `/skills list`: Displays all discovered skills and their status.
  * `/skills disable <name>`: Prevents a skill from being used (defaults to user scope).
  * `/skills enable <name>`: Re-enables a disabled skill (defaults to user scope).
  * `/skills reload`: Refreshes the list of discovered skills.
  * `--scope workspace` option for workspace-specific management.
* **FR3.6 - Skill Management (Terminal):** Users MUST be able to manage skills via `gemini skills` CLI commands (list, install, uninstall, enable, disable), supporting various installation sources (Git repo, local directory, `.skill` file).
* **FR3.7 - Progressive Disclosure:** Only skill metadata (`name`, `description`) MUST be loaded initially into the agent's context. Full instructions and resources MUST ONLY be injected when the agent explicitly calls the `activate_skill` tool.
* **FR3.8 - Activation & Consent:**
  * When the agent calls `activate_skill`, the user MUST be presented with a confirmation prompt detailing the skill's name, purpose, and the directory path it will access.
  * Upon user approval, the `SKILL.md` body and folder structure MUST be added to the conversation history, and the skill's directory MUST be added to the agent's allowed file paths.

**Technical Considerations (Integration with `zero-mcp-gemini`):**

* Skills offer an orchestration layer for the tools provided by `zero-mcp-gemini`. For example, a "Code Review" skill could define a workflow:
    1. Use `getProjectResources` to list relevant files.
    2. Use `getComponents` to identify React components.
    3. Use `read_file` (a core CLI tool) to read component code.
    4. Pass the code and relevant project analysis from `analyze_project` (from `zero-mcp-gemini`) into a specialized prompt.
    5. Use `write_file` (a core CLI tool) to record review findings.
* The prompts registered by `zero-mcp-gemini` (e.g., `code_review`, `refactor_suggest`) can be explicitly invoked within a skill's procedural guidance.
* `zero-mcp-gemini` could also bundle its own pre-defined skills as "Extension Skills," offering out-of-the-box advanced workflows related to its analytical capabilities. This would require defining `SKILL.md` files within the extension's structure.

## 5. User Experience

* **Intuitive Command Invocation:** Custom commands should feel like native CLI commands.
* **Clear Policy Feedback:** Users should understand why a tool execution was allowed, denied, or required confirmation.
* **Transparent Skill Activation:** Users should always be informed and grant consent before a skill is fully activated and gains additional access.
* **Discoverability:** `gemini skills list` and `/help` for commands should make available extensions clear.

## 6. Security & Compliance

* **Least Privilege:** Agent access to tools and file systems SHOULD always adhere to the principle of least privilege, managed by the Policy Engine.
* **User Consent:** Critical operations (shell commands, skill activation) MUST always require explicit user consent.
* **Secure Argument Handling:** Automatic shell escaping for arguments injected into shell commands is paramount to prevent command injection vulnerabilities.
* **Policy Integrity:** Policies, especially admin-defined ones, MUST be tamper-resistant and clearly auditable.

## 7. Future Considerations

* **Skill Versioning:** Mechanisms for skill versioning and compatibility checks.
* **Enhanced Argument Types:** Beyond simple strings, support for more structured argument types in custom commands.
* **Policy Language Extensions:** More expressive conditions in the policy language.
* **Skill Marketplace/Registry:** A centralized system for discovering, installing, and sharing skills.
