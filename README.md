# Buildkite MCP Extension for Zed

This project integrates Buildkite CI/CD functionality directly into the [Zed editor](https://zed.dev) using the [Model Context Protocol (MCP)](https://modelcontextprotocol.io), enabling AI-powered interaction with your Buildkite pipelines from within Zed.

## Features

- View Buildkite pipelines and organization information
- List and inspect builds with filtering
- View detailed job logs
- Download and inspect build artifacts
- Seamless integration with Zed's AI assistant

## Prerequisites

- [Zed editor](https://zed.dev/download)
- [Buildkite API token](https://buildkite.com/user/api-access-tokens) ([create one here](https://buildkite.com/user/api-access-tokens))
- [Docker](https://www.docker.com) (recommended for MCP server)
- [Go](https://golang.org) (alternative for MCP server)
- [Rust](https://rustup.rs) (for building extension from source)

## Installation

### 1. Install the Zed Extension

**Option A: Pre-built Extension (Easiest)**

1. Download the latest release from the [Releases](https://github.com/mcncl/zed-mcp-server-buildkite/releases) page
2. In Zed, select `File > Extensions > Install From Path` and choose the `.wasm` file

**Option B: Build from Source**

1. Clone this repository:
   ```sh
   git clone https://github.com/mcncl/zed-mcp-server-buildkite.git
   cd zed-mcp-server-buildkite
   ```
2. Build the extension:
   ```sh
   cargo build --target wasm32-wasi --release
   ```
3. In Zed, select `File > Extensions > Install From Path` and choose the built `.wasm` file

### 2. Install the MCP Server

**Option A: Docker (Recommended)**

```sh
docker pull ghcr.io/buildkite/buildkite-mcp-server
```

**Option B: Local Installation**

```sh
go install github.com/buildkite/buildkite-mcp-server/cmd/buildkite-mcp-server@latest
```

Or download a prebuilt binary from the [releases page](https://github.com/buildkite/buildkite-mcp-server/releases).

## Configuration

1. Go to your Buildkite account's Developer Settings and [create a Personal Access Token](https://buildkite.com/user/api-access-tokens).
2. In your project root, create a `.zed/settings.json` file with:

```json
{
  "context_servers": {
    "buildkite-mcp": {
      "settings": {
        "buildkite_api_token": "your-buildkite-token-here",
        "use_docker": true
      }
    }
  }
}
```

- Set `use_docker` to `false` if you want to use a locally installed binary instead of Docker.

## Usage

Once installed and configured, you can access Buildkite functionality through Zed's AI features:

- Use `/buildkite` commands or natural language queries, for example:
  - "Show me recent builds for my pipeline"
  - "Display logs for my failed job"
  - "List my organization's pipelines"

## Available Tools

- `get_pipeline` — Get details of a specific pipeline
- `list_pipelines` — List all pipelines in your organization
- `list_builds` — List all builds in a pipeline
- `get_job_logs` — Get logs for a specific job
- `list_artifacts` — List artifacts for a job
- `get_artifact` — Download a specific artifact
- `current_user` — Get current user details
- `user_token_organization` — Get organization info for your token

## Development

1. Make your changes to the Rust code in `src/`
2. Build with:
   ```sh
   cargo build --target wasm32-wasi --release
   ```
3. In Zed, install the updated `.wasm` file via `File > Extensions > Install From Path`

## Troubleshooting

- Ensure your Buildkite API token is valid and has the necessary permissions
- If using Docker, make sure Docker is running on your system
- If using local installation, ensure that the `buildkite-mcp-server` binary is in your PATH
- Check Zed's console logs (View > Developer > Toggle Console) for errors
- Verify the contents of your `.zed/settings.json` file

## License

MIT License — See the [LICENSE](./LICENSE) file for details.

## Contributing & Help

Contributions are welcome! Please feel free to submit a Pull Request or open an issue on the [GitHub repository](https://github.com/mcncl/zed-mcp-server-buildkite/issues).
