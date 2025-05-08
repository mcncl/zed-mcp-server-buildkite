use schemars::JsonSchema;
use serde::Deserialize;
use std::fs;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const REPO_NAME: &str = "buildkite/buildkite-mcp-server";
const BINARY_NAME: &str = "buildkite-mcp-server";

#[derive(Debug, Deserialize, JsonSchema)]
struct BuildkiteContextServerSettings {
    buildkite_api_token: String,
}

struct BuildkiteMCPExtension {
    cached_binary_path: Option<String>,
}

impl BuildkiteMCPExtension {
    fn context_server_binary_path(
        &mut self,
        _context_server_id: &ContextServerId,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        let release = zed::latest_github_release(
            REPO_NAME,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "{BINARY_NAME}_{os}_{arch}.{ext}",
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X86 => "i386",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "Darwin",
                zed::Os::Linux => "Linux",
                zed::Os::Windows => "Windows",
            },
            ext = match platform {
                zed::Os::Mac | zed::Os::Linux => "tar.gz",
                zed::Os::Windows => "zip",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("{BINARY_NAME}-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;
        let binary_path = format!("{}/{}", version_dir, BINARY_NAME);

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            let file_kind = match platform {
                zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                zed::Os::Windows => zed::DownloadedFileType::Zip,
            };

            zed::download_file(&asset.download_url, &version_dir, file_kind)
                .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            // Removes old versions
            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for BuildkiteMCPExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        eprintln!("Buildkite MCP: context_server_command called");
        let settings = ContextServerSettings::for_project("mcp-server-buildkite", project)?;
        let Some(settings) = settings.settings else {
            eprintln!("Buildkite MCP: missing `buildkite_api_token` setting");
            return Err("missing `buildkite_api_token` setting".into());
        };
        let settings: BuildkiteContextServerSettings = match serde_json::from_value(settings) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Buildkite MCP: error parsing settings: {}", e);
                return Err(e.to_string());
            }
        };
        eprintln!("Buildkite MCP: parsed settings: {:?}", settings);

        let binary_path = self.context_server_binary_path(_context_server_id)?;
        eprintln!("Buildkite MCP: launching binary at {}", binary_path);

        Ok(Command {
            command: binary_path,
            args: vec!["stdio".to_string()],
            env: vec![("BUILDKITE_API_TOKEN".into(), settings.buildkite_api_token)],
        })
    }
}

zed::register_extension!(BuildkiteMCPExtension);
