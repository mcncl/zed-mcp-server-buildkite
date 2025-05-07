# Installing the Buildkite MCP Extension for Zed

This guide will walk you through installing and configuring the Buildkite MCP extension for Zed.

## Prerequisites

1. [Zed editor](https://zed.dev/download) installed on your system
2. [Buildkite API token](https://buildkite.com/user/api-access-tokens) with appropriate permissions
3. Rust toolchain with wasm32-wasi target (for building from source)
4. Docker (recommended) or Go (alternative) for running the MCP server

## Installation Methods

### Method 1: Install Pre-built Extension (Easiest)

1. Download the latest release from the [Releases](https://github.com/mcncl/zed-mcp-server-buildkite/releases) page
2. In Zed, select `File > Extensions > Install From Path`
3. Select the downloaded `.wasm` file

### Method 2: Build from Source

1. Clone this repository:
   ```
   git clone https://github.com/mcncl/zed-mcp-server-buildkite.git
   cd zed-mcp-server-buildkite
   ```

2. Run the build script:
   ```
   ./build.sh
   ```

3. In Zed, select `File > Extensions > Install From Path`
4. Select the `dist/zed_buildkite_mcp.wasm` file

## Install the MCP Server

You can use one of the following methods to install the Buildkite MCP server:

### Option 1: Docker (Recommended)

The easiest way is to use Docker:

```
docker pull ghcr.io/buildkite/buildkite-mcp-server
```

This requires Docker to be installed on your system but doesn't require Go or any other dependencies.

### Option 2: Local Installation

Alternatively, you can install the Buildkite MCP server directly:

```
go install github.com/buildkite/buildkite-mcp-server/cmd/buildkite-mcp-server@latest
```

Or download a prebuilt binary from the [releases page](https://github.com/buildkite/buildkite-mcp-server/releases).

If using this method, ensure that the `buildkite-mcp-server` binary is in your PATH.

## Configuration

1. Create a `.zed` directory in your project root if it doesn't exist:
   ```
   mkdir -p .zed
   ```

2. Create a `settings.json` file in the `.zed` directory:
   ```
   touch .zed/settings.json
   ```

3. Add the following content to `settings.json`, replacing `your-buildkite-token-here` with your actual Buildkite API token:
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

   Set `use_docker` to `false` if you want to use a locally installed binary instead of Docker.

## Verification

To verify that your extension is working:

1. Open the Zed console with `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)
2. Type "MCP" and select "Show MCP Servers"
3. You should see "buildkite-mcp" in the list of available MCP servers

## Troubleshooting

If the extension is not working:

1. Check that your Buildkite API token is valid and has the necessary permissions
2. If using Docker, make sure Docker is running on your system
3. If using local installation, ensure that the `buildkite-mcp-server` binary is in your PATH
4. Look for errors in the Zed console logs (View > Developer > Toggle Console)
5. Verify the contents of your `.zed/settings.json` file

## Getting Help

If you encounter issues, please open an issue on the [GitHub repository](https://github.com/mcncl/zed-mcp-server-buildkite/issues).
