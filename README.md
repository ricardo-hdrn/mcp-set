# mcp-set

Install MCP servers into AI client configurations. Rust library + CLI.

Supports 4 package managers (npm, pip, go, cargo) and 10 AI clients.

## Supported Clients

| Client | Config Format | Global | Local |
|--------|:------------:|:------:|:-----:|
| Claude Code | JSON | yes | yes |
| Claude Desktop | JSON | yes | — |
| Codex | TOML | yes | yes |
| Cursor | JSON | yes | yes |
| Gemini CLI | JSON | yes | yes |
| Goose | YAML | yes | — |
| GitHub Copilot | JSON | yes | yes |
| OpenCode | JSON | yes | yes |
| VS Code | JSON | yes | yes |
| Zed | JSON | yes | yes |

## CLI Usage

```bash
# Install a local binary
mcp-set install /path/to/mcp-server -a claude-code -g

# Install a URL endpoint
mcp-set install https://example.com/mcp -a vscode -a cursor -g

# Install an npm package (auto-detected)
mcp-set install @org/mcp-server -a claude-code -g

# Install a pip package
mcp-set install mcp-server-fetch --from pip -a claude-code -g

# Install a Go module (auto-detected)
mcp-set install github.com/user/mcp -a claude-code -g

# Install a cargo binary
mcp-set install my-mcp --from cargo -a claude-code -g

# Install to all agents
mcp-set install /path/to/server --all -g

# With env vars and extra args
mcp-set install /path/to/server -a claude-code -g -e API_KEY=secret -- --verbose

# List supported agents
mcp-set list-agents

# Detect installed agents
mcp-set detect
mcp-set detect --local
```

## Library Usage

```rust
use mcp_set::{install_command, Agent, Scope};

let binary = std::env::current_exe().unwrap();
let results = install_command(
    "my-server",
    binary.to_str().unwrap(),
    &[],
    &[Agent::ClaudeCode, Agent::Cursor],
    Scope::Global,
);

for result in results {
    match result {
        Ok(r) => println!("Installed to {} at {}", r.agent, r.path),
        Err(e) => eprintln!("Error: {e}"),
    }
}
```

As a dependency (library only, no CLI):

```toml
[dependencies]
mcp-set = { version = "0.2", default-features = false }
```

## License

MIT
