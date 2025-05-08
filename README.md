# Buildkite MCP Extension for Zed

Integrate Buildkite CI/CD with the [Zed editor](https://zed.dev) using the Model Context Protocol (MCP). This extension lets you view and interact with Buildkite pipelines, builds, jobs, and artifacts directly from Zed.

## Features

- View Buildkite pipelines and builds
- Inspect jobs and logs
- Download build artifacts
- Seamless integration with Zed's AI features

## Requirements

- Buildkite API token ([create one here](https://buildkite.com/user/api-access-tokens))

## Usage

After installing the extension from the Zed marketplace, configure your Buildkite API token in Zed's settings. The extension will handle all server and connection details automatically.

```json
{
  "context_servers": {
    "mcp-server-buildkite": {
      "settings": {
        "buildkite_api_token": "your-buildkite-token-here",
      }
    }
  }
}
```

Example queries:
- "Show me recent builds for my pipeline"
- "Display logs for my failed job"
- "List my organization's pipelines"

## License

MIT License — See the [LICENSE](./LICENSE) file for details.
