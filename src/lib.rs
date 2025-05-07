use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

#[derive(Debug, Deserialize)]
struct BuildkiteContextServerSettings {
    buildkite_api_token: String,
    use_docker: Option<bool>,
}

struct BuildkiteMCPExtension;

impl zed::Extension for BuildkiteMCPExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("buildkite-mcp", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `buildkite_api_token` setting".into());
        };
        
        let settings: BuildkiteContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;
        
        // Check if we should use Docker (default to true for easier setup)
        let use_docker = settings.use_docker.unwrap_or(true);
        
        if use_docker {
            // Use Docker to run the MCP server
            Ok(Command {
                command: "docker".to_string(),
                args: vec![
                    "run".to_string(),
                    "-i".to_string(),
                    "--rm".to_string(),
                    "-e".to_string(),
                    "BUILDKITE_API_TOKEN".to_string(),
                    "ghcr.io/buildkite/buildkite-mcp-server".to_string(),
                    "stdio".to_string(),
                ],
                env: vec![("BUILDKITE_API_TOKEN".into(), settings.buildkite_api_token)],
            })
        } else {
            // Find the local buildkite-mcp-server binary
            let binary_path = match env::consts::OS {
                "windows" => "buildkite-mcp-server.exe",
                _ => "buildkite-mcp-server",
            };

            Ok(Command {
                command: binary_path.to_string(),
                args: vec!["stdio".to_string()],
                env: vec![("BUILDKITE_API_TOKEN".into(), settings.buildkite_api_token)],
            })
        }
    }
}

zed::register_extension!(BuildkiteMCPExtension);