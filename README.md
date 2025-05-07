# Buildkite MCP Extension for Zed

This extension integrates Buildkite CI/CD functionality directly into the [Zed editor](https://zed.dev) using the [Model Context Protocol (MCP)](https://modelcontextprotocol.io).

## Features

- View Buildkite pipelines
- List and inspect builds
- View job logs
- Download build artifacts
- Get user and organization information

## Installation

### Prerequisites

1. [Zed editor](https://zed.dev/download)
2. [Buildkite API token](https://buildkite.com/user/api-access-tokens) with appropriate permissions

### Install the Extension

1. Clone this repository
2. Build the extension:
   ```
   cd zed-mcp-server-buildkite
   cargo build --target wasm32-wasi --release
   ```
3. Install in Zed using the Extension panel: `File > Extensions > Install From Path`

### Install the MCP Server

You can use the Buildkite MCP server in two ways:

#### Option 1: Docker (Recommended)

The easiest way is to use Docker, which requires no installation of Go or building from source:

```
docker pull ghcr.io/buildkite/buildkite-mcp-server
```

The Zed extension will automatically use Docker by default.

#### Option 2: Local Binary

Alternatively, you can install the server locally:

```
go install github.com/buildkite/buildkite-mcp-server/cmd/buildkite-mcp-server@latest
```

Or download a prebuilt binary from the [releases page](https://github.com/buildkite/buildkite-mcp-server/releases).

## Configuration

In Zed, create a `.zed/settings.json` file in your project with:

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

Set `use_docker` to `false` if you want to use a locally installed binary instead.

## Usage

With the extension installed and configured, you can access Buildkite functionality through Zed's AI features:

- Use `/buildkite` commands or natural language questions to interact with Buildkite
- Example queries:
  - "Show me recent builds for my pipeline"
  - "Display logs for my failed job"
  - "List my organization's pipelines"

## Available Tools

- `get_pipeline` - Get details of a specific pipeline
- `list_pipelines` - List all pipelines in your organization
- `list_builds` - List all builds in a pipeline
- `get_job_logs` - Get logs for a specific job
- `list_artifacts` - List artifacts for a job
- `get_artifact` - Download a specific artifact
- `current_user` - Get current user details
- `user_token_organization` - Get organization info for your token

## Development

To develop this extension:

1. Make your changes to the Rust code
2. Build with: `cargo build --target wasm32-wasi --release`
3. Test in Zed using the local installation method

## License

MIT License - See the [LICENSE](./LICENSE) file for details.
