# Gemini CLI extensions

Gemini CLI extensions package prompts, MCP servers, custom commands, hooks, and
agent skills into a familiar and user-friendly format. With extensions, you can
expand the capabilities of Gemini CLI and share those capabilities with others.
They are designed to be easily installable and shareable.

To see examples of extensions, you can browse a gallery of
[Gemini CLI extensions](https://geminicli.com/extensions/browse/).

## Managing extensions

You can verify your installed extensions and their status using the interactive
command:

```bash
/extensions list
```

or in noninteractive mode:

```bash
gemini extensions list
```

## Installation

To install a real extension, you can use the `extensions install` command with a
GitHub repository URL in noninteractive mode. For example:

```bash
gemini extensions install https://github.com/gemini-cli-extensions/workspace
```

---

# Getting started with Gemini CLI extensions

This guide will walk you through creating your first Gemini CLI extension.
You'll learn how to set up a new extension, add a custom tool via an MCP server,
create a custom command, and provide context to the model with a `GEMINI.md`
file.

## Prerequisites

Before you start, make sure you have the Gemini CLI installed and a basic
understanding of Node.js.

## When to use what

Extensions offer a variety of ways to customize Gemini CLI.

| Feature                                                                      | What it is                                                                                                         | When to use it                                                                                                                                                                                                                                                                                 | Invoked by            |
| :--------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------- |
| **[MCP server](/docs/extensions/reference#mcp-servers)**                     | A standard way to expose new tools and data sources to the model.                                                  | Use this when you want the model to be able to _do_ new things, like fetching data from an internal API, querying a database, or controlling a local application. We also support MCP resources (which can replace custom commands) and system instructions (which can replace custom context) | Model                 |
| **[Custom commands](/docs/cli/custom-commands)**                             | A shortcut (like `/my-cmd`) that executes a pre-defined prompt or shell command.                                   | Use this for repetitive tasks or to save long, complex prompts that you use frequently. Great for automation.                                                                                                                                                                                  | User                  |
| **[Context file (`GEMINI.md`)](/docs/extensions/reference#contextfilename)** | A markdown file containing instructions that are loaded into the model's context at the start of every session.    | Use this to define the "personality" of your extension, set coding standards, or provide essential knowledge that the model should always have.                                                                                                                                                | CLI provides to model |
| **[Agent skills](/docs/cli/skills)**                                         | A specialized set of instructions and workflows that the model activates only when needed.                         | Use this for complex, occasional tasks (like "create a PR" or "audit security") to avoid cluttering the main context window when the skill isn't being used.                                                                                                                                   | Model                 |
| **[Hooks](/docs/hooks)**                                                     | A way to intercept and customize the CLI's behavior at specific lifecycle events (e.g., before/after a tool call). | Use this when you want to automate actions based on what the model is doing, like validating tool arguments, logging activity, or modifying the model's input/output.                                                                                                                          | CLI                   |

## Step 1: Create a new extension

The easiest way to start is by using one of the built-in templates. We'll use
the `mcp-server` example as our foundation.

Run the following command to create a new directory called `my-first-extension`
with the template files:

```bash
gemini extensions new my-first-extension mcp-server
```

This will create a new directory with the following structure:

```
my-first-extension/
├── example.js
├── gemini-extension.json
└── package.json
```

## Step 2: Understand the extension files

Let's look at the key files in your new extension.

### `gemini-extension.json`

This is the manifest file for your extension. It tells Gemini CLI how to load
and use your extension.

```json
{
  "name": "mcp-server-example",
  "version": "1.0.0",
  "mcpServers": {
    "nodeServer": {
      "command": "node",
      "args": ["${extensionPath}${/}example.js"],
      "cwd": "${extensionPath}"
    }
  }
}
```

- `name`: The unique name for your extension.
- `version`: The version of your extension.
- `mcpServers`: This section defines one or more Model Context Protocol (MCP)
  servers. MCP servers are how you can add new tools for the model to use.
  - `command`, `args`, `cwd`: These fields specify how to start your server.
    Notice the use of the `${extensionPath}` variable, which Gemini CLI replaces
    with the absolute path to your extension's installation directory. This
    allows your extension to work regardless of where it's installed.

### `example.js`

This file contains the source code for your MCP server. It's a simple Node.js
server that uses the `@modelcontextprotocol/sdk`.

```javascript
/**
 * @license
 * Copyright 2025 Google LLC
 * SPDX-License-Identifier: Apache-2.0
 */

import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { z } from 'zod';

const server = new McpServer({
  name: 'prompt-server',
  version: '1.0.0',
});

// Registers a new tool named 'fetch_posts'
server.registerTool(
  'fetch_posts',
  {
    description: 'Fetches a list of posts from a public API.',
    inputSchema: z.object({}).shape,
  },
  async () => {
    const apiResponse = await fetch(
      'https://jsonplaceholder.typicode.com/posts',
    );
    const posts = await apiResponse.json();
    const response = { posts: posts.slice(0, 5) };
    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify(response),
        },
      ],
    };
  },
);

// ... (prompt registration omitted for brevity)

const transport = new StdioServerTransport();
await server.connect(transport);
```

This server defines a single tool called `fetch_posts` that fetches data from a
public API.

### `package.json`

This is the standard configuration file for a Node.js project. It defines
dependencies and scripts.

## Step 3: Link your extension

Before you can use the extension, you need to link it to your Gemini CLI
installation for local development.

1. **Install dependencies:**

    ```bash
    cd my-first-extension
    npm install
    ```

2. **Link the extension:**

    The `link` command creates a symbolic link from the Gemini CLI extensions
    directory to your development directory. This means any changes you make
    will be reflected immediately without needing to reinstall.

    ```bash
    gemini extensions link .
    ```

Now, restart your Gemini CLI session. The new `fetch_posts` tool will be
available. You can test it by asking: "fetch posts".

## Step 4: Add a custom command

Custom commands provide a way to create shortcuts for complex prompts. Let's add
a command that searches for a pattern in your code.

1. Create a `commands` directory and a subdirectory for your command group:

    ```bash
    mkdir -p commands/fs
    ```

2. Create a file named `commands/fs/grep-code.toml`:

    ```toml
    prompt = """
    Please summarize the findings for the pattern `{{args}}`.

    Search Results:
    !{grep -r {{args}} .}
    """
    ```

    This command, `/fs:grep-code`, will take an argument, run the `grep` shell
    command with it, and pipe the results into a prompt for summarization.

After saving the file, restart the Gemini CLI. You can now run
`/fs:grep-code "some pattern"` to use your new command.

## Step 5: Add a custom `GEMINI.md`

You can provide persistent context to the model by adding a `GEMINI.md` file to
your extension. This is useful for giving the model instructions on how to
behave or information about your extension's tools. Note that you may not always
need this for extensions built to expose commands and prompts.

1. Create a file named `GEMINI.md` in the root of your extension directory:

    ```markdown
    # My First Extension Instructions

    You are an expert developer assistant. When the user asks you to fetch
    posts, use the `fetch_posts` tool. Be concise in your responses.
    ```

2. Update your `gemini-extension.json` to tell the CLI to load this file:

    ```json
    {
      "name": "my-first-extension",
      "version": "1.0.0",
      "contextFileName": "GEMINI.md",
      "mcpServers": {
        "nodeServer": {
          "command": "node",
          "args": ["${extensionPath}${/}example.js"],
          "cwd": "${extensionPath}"
        }
      }
    }
    ```

Restart the CLI again. The model will now have the context from your `GEMINI.md`
file in every session where the extension is active.

## (Optional) Step 6: Add an Agent Skill

_Note: This is an experimental feature enabled via `experimental.skills`._

[Agent Skills](/docs/cli/skills) let you bundle specialized expertise and
procedural workflows. Unlike `GEMINI.md`, which provides persistent context,
skills are activated only when needed, saving context tokens.

1. Create a `skills` directory and a subdirectory for your skill:

    ```bash
    mkdir -p skills/security-audit
    ```

2. Create a `skills/security-audit/SKILL.md` file:

    ```markdown
    ---
    name: security-audit
    description:
      Expertise in auditing code for security vulnerabilities. Use when the user
      asks to "check for security issues" or "audit" their changes.
    ---

    # Security Auditor

    You are an expert security researcher. When auditing code:

    1. Look for common vulnerabilities (OWASP Top 10).
    2. Check for hardcoded secrets or API keys.
    3. Suggest remediation steps for any findings.
    ```

Skills bundled with your extension are automatically discovered and can be
activated by the model during a session when it identifies a relevant task.

## Step 7: Release your extension

Once you're happy with your extension, you can share it with others. The two
primary ways of releasing extensions are via a Git repository or through GitHub
Releases. Using a public Git repository is the simplest method.

For detailed instructions on both methods, please refer to the
[Extension Releasing Guide](/docs/extensions/releasing).

## Conclusion

You've successfully created a Gemini CLI extension! You learned how to:

- Bootstrap a new extension from a template.
- Add custom tools with an MCP server.
- Create convenient custom commands.
- Provide persistent context to the model.
- Bundle specialized Agent Skills.
- Link your extension for local development.

From here, you can explore more advanced features and build powerful new
capabilities into the Gemini CLI.

---

# Extensions reference

This guide covers the `gemini extensions` commands and the structure of the
`gemini-extension.json` configuration file.

## Extension management

We offer a suite of extension management tools using `gemini extensions`
commands.

Note that these commands (e.g. `gemini extensions install`) are not supported
from within the CLI's **interactive mode**, although you can list installed
extensions using the `/extensions list` slash command.

Note that all of these management operations (including updates to slash
commands) will only be reflected in active CLI sessions on **restart**.

### Installing an extension

You can install an extension using `gemini extensions install` with either a
GitHub URL or a local path.

Note that we create a copy of the installed extension, so you will need to run
`gemini extensions update` to pull in changes from both locally-defined
extensions and those on GitHub.

NOTE: If you are installing an extension from GitHub, you'll need to have `git`
installed on your machine. See
[git installation instructions](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
for help.

```
gemini extensions install <source> [--ref <ref>] [--auto-update] [--pre-release] [--consent]
```

- `<source>`: The github URL or local path of the extension to install.
- `--ref`: The git ref to install from.
- `--auto-update`: Enable auto-update for this extension.
- `--pre-release`: Enable pre-release versions for this extension.
- `--consent`: Acknowledge the security risks of installing an extension and
  skip the confirmation prompt.

### Uninstalling an extension

To uninstall one or more extensions, run
`gemini extensions uninstall <name...>`:

```
gemini extensions uninstall gemini-cli-security gemini-cli-another-extension
```

### Disabling an extension

Extensions are, by default, enabled across all workspaces. You can disable an
extension entirely or for specific workspace.

```
gemini extensions disable <name> [--scope <scope>]
```

- `<name>`: The name of the extension to disable.
- `--scope`: The scope to disable the extension in (`user` or `workspace`).

### Enabling an extension

You can enable extensions using `gemini extensions enable <name>`. You can also
enable an extension for a specific workspace using
`gemini extensions enable <name> --scope=workspace` from within that workspace.

```
gemini extensions enable <name> [--scope <scope>]
```

- `<name>`: The name of the extension to enable.
- `--scope`: The scope to enable the extension in (`user` or `workspace`).

### Updating an extension

For extensions installed from a local path or a git repository, you can
explicitly update to the latest version (as reflected in the
`gemini-extension.json` `version` field) with `gemini extensions update <name>`.

You can update all extensions with:

```
gemini extensions update --all
```

### Create a boilerplate extension

We offer several example extensions `context`, `custom-commands`,
`exclude-tools` and `mcp-server`. You can view these examples
[here](https://github.com/google-gemini/gemini-cli/tree/main/packages/cli/src/commands/extensions/examples).

To copy one of these examples into a development directory using the type of
your choosing, run:

```
gemini extensions new <path> [template]
```

- `<path>`: The path to create the extension in.
- `[template]`: The boilerplate template to use.

### Link a local extension

The `gemini extensions link` command will create a symbolic link from the
extension installation directory to the development path.

This is useful so you don't have to run `gemini extensions update` every time
you make changes you'd like to test.

```
gemini extensions link <path>
```

- `<path>`: The path of the extension to link.

## Extension format

On startup, Gemini CLI looks for extensions in `<home>/.gemini/extensions`

Extensions exist as a directory that contains a `gemini-extension.json` file.
For example:

`<home>/.gemini/extensions/my-extension/gemini-extension.json`

### `gemini-extension.json`

The `gemini-extension.json` file contains the configuration for the extension.
The file has the following structure:

```json
{
  "name": "my-extension",
  "version": "1.0.0",
  "description": "My awesome extension",
  "mcpServers": {
    "my-server": {
      "command": "node my-server.js"
    }
  },
  "contextFileName": "GEMINI.md",
  "excludeTools": ["run_shell_command"]
}
```

- `name`: The name of the extension. This is used to uniquely identify the
  extension and for conflict resolution when extension commands have the same
  name as user or project commands. The name should be lowercase or numbers and
  use dashes instead of underscores or spaces. This is how users will refer to
  your extension in the CLI. Note that we expect this name to match the
  extension directory name.
- `version`: The version of the extension.
- `description`: A short description of the extension. This will be displayed on
  [geminicli.com/extensions](https://geminicli.com/extensions).
- `mcpServers`: A map of MCP servers to settings. The key is the name of the
  server, and the value is the server configuration. These servers will be
  loaded on startup just like MCP servers settingsd in a
  [`settings.json` file](/docs/get-started/configuration). If both an extension
  and a `settings.json` file settings an MCP server with the same name, the
  server defined in the `settings.json` file takes precedence.
  - Note that all MCP server configuration options are supported except for
    `trust`.
- `contextFileName`: The name of the file that contains the context for the
  extension. This will be used to load the context from the extension directory.
  If this property is not used but a `GEMINI.md` file is present in your
  extension directory, then that file will be loaded.
- `excludeTools`: An array of tool names to exclude from the model. You can also
  specify command-specific restrictions for tools that support it, like the
  `run_shell_command` tool. For example,
  `"excludeTools": ["run_shell_command(rm -rf)"]` will block the `rm -rf`
  command. Note that this differs from the MCP server `excludeTools`
  functionality, which can be listed in the MCP server config.

When Gemini CLI starts, it loads all the extensions and merges their
configurations. If there are any conflicts, the workspace configuration takes
precedence.

### Settings

_Note: This is an experimental feature. We do not yet recommend extension
authors introduce settings as part of their core flows._

Extensions can define settings that the user will be prompted to provide upon
installation. This is useful for things like API keys, URLs, or other
configuration that the extension needs to function.

To define settings, add a `settings` array to your `gemini-extension.json` file.
Each object in the array should have the following properties:

- `name`: A user-friendly name for the setting.
- `description`: A description of the setting and what it's used for.
- `envVar`: The name of the environment variable that the setting will be stored
  as.
- `sensitive`: Optional boolean. If true, obfuscates the input the user provides
  and stores the secret in keychain storage. **Example**

```json
{
  "name": "my-api-extension",
  "version": "1.0.0",
  "settings": [
    {
      "name": "API Key",
      "description": "Your API key for the service.",
      "envVar": "MY_API_KEY"
    }
  ]
}
```

When a user installs this extension, they will be prompted to enter their API
key. The value will be saved to a `.env` file in the extension's directory
(e.g., `<home>/.gemini/extensions/my-api-extension/.env`).

You can view a list of an extension's settings by running:

```
gemini extensions list
```

and you can update a given setting using:

```
gemini extensions config <extension name> [setting name] [--scope <scope>]
```

- `--scope`: The scope to set the setting in (`user` or `workspace`). This is
  optional and will default to `user`.

### Custom commands

Extensions can provide [custom commands](/docs/cli/custom-commands) by placing
TOML files in a `commands/` subdirectory within the extension directory. These
commands follow the same format as user and project custom commands and use
standard naming conventions.

**Example**

An extension named `gcp` with the following structure:

```
.gemini/extensions/gcp/
├── gemini-extension.json
└── commands/
    ├── deploy.toml
    └── gcs/
        └── sync.toml
```

Would provide these commands:

- `/deploy` - Shows as `[gcp] Custom command from deploy.toml` in help
- `/gcs:sync` - Shows as `[gcp] Custom command from sync.toml` in help

### Hooks

Extensions can provide [hooks](/docs/hooks) to intercept and customize
Gemini CLI behavior at specific lifecycle events. Hooks provided by an extension
must be defined in a `hooks/hooks.json` file within the extension directory.

> [!IMPORTANT] Hooks are not defined directly in `gemini-extension.json`. The
> CLI specifically looks for the `hooks/hooks.json` file.

### Agent Skills

Extensions can bundle [Agent Skills](/docs/cli/skills) to provide specialized
workflows. Skills must be placed in a `skills/` directory within the extension.

**Example**

An extension with the following structure:

```
.gemini/extensions/my-extension/
├── gemini-extension.json
└── skills/
    └── security-audit/
        └── SKILL.md
```

Will expose a `security-audit` skill that the model can activate.

### Conflict resolution

Extension commands have the lowest precedence. When a conflict occurs with user
or project commands:

1. **No conflict**: Extension command uses its natural name (e.g., `/deploy`)
2. **With conflict**: Extension command is renamed with the extension prefix
   (e.g., `/gcp.deploy`)

For example, if both a user and the `gcp` extension define a `deploy` command:

- `/deploy` - Executes the user's deploy command
- `/gcp.deploy` - Executes the extension's deploy command (marked with `[gcp]`
  tag)

## Variables

Gemini CLI extensions allow variable substitution in `gemini-extension.json`.
This can be useful if e.g., you need the current directory to run an MCP server
using an argument like `"args": ["${extensionPath}${/}dist${/}server.js"]`.

**Supported variables:**

| variable                   | description                                                                                                                                                     |
| -------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `${extensionPath}`         | The fully-qualified path of the extension in the user's filesystem e.g., '/Users/username/.gemini/extensions/example-extension'. This will not unwrap symlinks. |
| `${workspacePath}`         | The fully-qualified path of the current workspace.                                                                                                              |
| `${/} or ${pathSeparator}` | The path separator (differs per OS).                                                                                                                            |

---

# Extensions on Gemini CLI: Best practices

This guide covers best practices for developing, securing, and maintaining
Gemini CLI extensions.

## Development

Developing extensions for Gemini CLI is intended to be a lightweight, iterative
process.

### Structure your extension

While simple extensions can just be a few files, we recommend a robust structure
for complex extensions:

```
my-extension/
├── package.json
├── tsconfig.json
├── gemini-extension.json
├── src/
│   ├── index.ts
│   └── tools/
└── dist/
```

- **Use TypeScript**: We strongly recommend using TypeScript for type safety and
  better tooling.
- **Separate source and build**: Keep your source code in `src` and build to
  `dist`.
- **Bundle dependencies**: If your extension has many dependencies, consider
  bundling them (e.g., with `esbuild` or `webpack`) to reduce install time and
  potential conflicts.

### Iterate with `link`

Use `gemini extensions link` to develop locally without constantly reinstalling:

```bash
cd my-extension
gemini extensions link .
```

Changes to your code (after rebuilding) will be immediately available in the CLI
on restart.

### Use `GEMINI.md` effectively

Your `GEMINI.md` file provides context to the model. Keep it focused:

- **Do:** Explain high-level goals and how to use the provided tools.
- **Don't:** Dump your entire documentation.
- **Do:** Use clear, concise language.

## Security

When building a Gemini CLI extension, follow general security best practices
(such as least privilege and input validation) to reduce risk.

### Minimal permissions

When defining tools in your MCP server, only request the permissions necessary.
Avoid giving the model broad access (like full shell access) if a more
restricted set of tools will suffice.

If you must use powerful tools like `run_shell_command`, consider restricting
them to specific commands in your `gemini-extension.json`:

```json
{
  "name": "my-safe-extension",
  "excludeTools": ["run_shell_command(rm -rf *)"]
}
```

This ensures that even if the model tries to execute a dangerous command, it
will be blocked at the CLI level.

### Validate inputs

Your MCP server is running on the user's machine. Always validate inputs to your
tools to prevent arbitrary code execution or filesystem access outside the
intended scope.

```typescript
// Good: Validating paths
if (!path.resolve(inputPath).startsWith(path.resolve(allowedDir) + path.sep)) {
  throw new Error('Access denied');
}
```

### Sensitive settings

If your extension requires API keys, use the `sensitive: true` option in
`gemini-extension.json`. This ensures keys are stored securely in the system
keychain and obfuscated in the UI.

```json
"settings": [
  {
    "name": "API Key",
    "envVar": "MY_API_KEY",
    "sensitive": true
  }
]
```

## Releasing

You can upload your extension directly to GitHub to list it in the gallery.
Gemini CLI extensions also offers support for more complicated
[releases](/docs/extensions/releasing).

### Semantic versioning

Follow [Semantic Versioning](https://semver.org/).

- **Major**: Breaking changes (renaming tools, changing arguments).
- **Minor**: New features (new tools, commands).
- **Patch**: Bug fixes.

### Release Channels

Use git branches to manage release channels (e.g., `main` for stable, `dev` for
bleeding edge). This allows users to choose their stability level:

```bash
# Stable
gemini extensions install github.com/user/repo

# Dev
gemini extensions install github.com/user/repo --ref dev
```

### Clean artifacts

If you are using GitHub Releases, ensure your release artifacts only contain the
necessary files (`dist/`, `gemini-extension.json`, `package.json`). Exclude
`node_modules` (users will install them) and `src/` to keep downloads small.

---

# Extension releasing

There are two primary ways of releasing extensions to users:

- [Git repository](#releasing-through-a-git-repository)
- [Github Releases](#releasing-through-github-releases)

Git repository releases tend to be the simplest and most flexible approach,
while GitHub releases can be more efficient on initial install as they are
shipped as single archives instead of requiring a git clone which downloads each
file individually. Github releases may also contain platform specific archives
if you need to ship platform specific binary files.

## Releasing through a git repository

This is the most flexible and simple option. All you need to do is create a
publicly accessible git repo (such as a public github repository) and then users
can install your extension using `gemini extensions install <your-repo-uri>`.
They can optionally depend on a specific ref (branch/tag/commit) using the
`--ref=<some-ref>` argument, this defaults to the default branch.

Whenever commits are pushed to the ref that a user depends on, they will be
prompted to update the extension. Note that this also allows for easy rollbacks,
the HEAD commit is always treated as the latest version regardless of the actual
version in the `gemini-extension.json` file.

### Managing release channels using a git repository

Users can depend on any ref from your git repo, such as a branch or tag, which
allows you to manage multiple release channels.

For instance, you can maintain a `stable` branch, which users can install this
way `gemini extensions install <your-repo-uri> --ref=stable`. Or, you could make
this the default by treating your default branch as your stable release branch,
and doing development in a different branch (for instance called `dev`). You can
maintain as many branches or tags as you like, providing maximum flexibility for
you and your users.

Note that these `ref` arguments can be tags, branches, or even specific commits,
which allows users to depend on a specific version of your extension. It is up
to you how you want to manage your tags and branches.

### Example releasing flow using a git repo

While there are many options for how you want to manage releases using a git
flow, we recommend treating your default branch as your "stable" release branch.
This means that the default behavior for
`gemini extensions install <your-repo-uri>` is to be on the stable release
branch.

Lets say you want to maintain three standard release channels, `stable`,
`preview`, and `dev`. You would do all your standard development in the `dev`
branch. When you are ready to do a preview release, you merge that branch into
your `preview` branch. When you are ready to promote your preview branch to
stable, you merge `preview` into your stable branch (which might be your default
branch or a different branch).

You can also cherry pick changes from one branch into another using
`git cherry-pick`, but do note that this will result in your branches having a
slightly divergent history from each other, unless you force push changes to
your branches on each release to restore the history to a clean slate (which may
not be possible for the default branch depending on your repository settings).
If you plan on doing cherry picks, you may want to avoid having your default
branch be the stable branch to avoid force-pushing to the default branch which
should generally be avoided.

## Releasing through GitHub releases

Gemini CLI extensions can be distributed through
[GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases).
This provides a faster and more reliable initial installation experience for
users, as it avoids the need to clone the repository.

Each release includes at least one archive file, which contains the full
contents of the repo at the tag that it was linked to. Releases may also include
[pre-built archives](#custom-pre-built-archives) if your extension requires some
build step or has platform specific binaries attached to it.

When checking for updates, gemini will just look for the "latest" release on
github (you must mark it as such when creating the release), unless the user
installed a specific release by passing `--ref=<some-release-tag>`.

You may also install extensions with the `--pre-release` flag in order to get
the latest release regardless of whether it has been marked as "latest". This
allows you to test that your release works before actually pushing it to all
users.

### Custom pre-built archives

Custom archives must be attached directly to the github release as assets and
must be fully self-contained. This means they should include the entire
extension, see [archive structure](#archive-structure).

If your extension is platform-independent, you can provide a single generic
asset. In this case, there should be only one asset attached to the release.

Custom archives may also be used if you want to develop your extension within a
larger repository, you can build an archive which has a different layout from
the repo itself (for instance it might just be an archive of a subdirectory
containing the extension).

#### Platform specific archives

To ensure Gemini CLI can automatically find the correct release asset for each
platform, you must follow this naming convention. The CLI will search for assets
in the following order:

1. **Platform and architecture-Specific:**
    `{platform}.{arch}.{name}.{extension}`
2. **Platform-specific:** `{platform}.{name}.{extension}`
3. **Generic:** If only one asset is provided, it will be used as a generic
    fallback.

- `{name}`: The name of your extension.
- `{platform}`: The operating system. Supported values are:
  - `darwin` (macOS)
  - `linux`
  - `win32` (Windows)
- `{arch}`: The architecture. Supported values are:
  - `x64`
  - `arm64`
- `{extension}`: The file extension of the archive (e.g., `.tar.gz` or `.zip`).

**Examples:**

- `darwin.arm64.my-tool.tar.gz` (specific to Apple Silicon Macs)
- `darwin.my-tool.tar.gz` (for all Macs)
- `linux.x64.my-tool.tar.gz`
- `win32.my-tool.zip`

#### Archive structure

Archives must be fully contained extensions and have all the standard
requirements - specifically the `gemini-extension.json` file must be at the root
of the archive.

The rest of the layout should look exactly the same as a typical extension, see
[extensions.md](/docs/extensions).

#### Example GitHub Actions workflow

Here is an example of a GitHub Actions workflow that builds and releases a
Gemini CLI extension for multiple platforms:

```yaml
name: Release Extension

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Set up Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '20'

      - name: Install dependencies
        run: npm ci

      - name: Build extension
        run: npm run build

      - name: Create release assets
        run: |
          npm run package -- --platform=darwin --arch=arm64
          npm run package -- --platform=linux --arch=x64
          npm run package -- --platform=win32 --arch=x64

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            release/darwin.arm64.my-tool.tar.gz
            release/linux.arm64.my-tool.tar.gz
            release/win32.arm64.my-tool.zip
```
