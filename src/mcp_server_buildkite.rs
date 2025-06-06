use schemars::JsonSchema;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};
use log::{info, warn, error};

// Custom Error Enum
#[derive(Debug)]
enum ExtensionError {
    ProjectOrSettingsMissing(String),
    SettingsParseError(serde_json::Error),
    IoError(std::io::Error),
    ZedError(String), // For errors from zed_extension_api
    AssetNotFound(String),
    UnsupportedPlatform(String), // Kept for future use, though asset_triplet_for_host was removed
    ReleaseNotFound(String),
    DownloadFailed(String),
    InternalError(String), // Generic catch-all
}

// Implement std::fmt::Display for ExtensionError
impl std::fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionError::ProjectOrSettingsMissing(msg) => {
                write!(f, "Project or settings missing: {}", msg)
            }
            ExtensionError::SettingsParseError(err) => write!(f, "Settings parse error: {}", err),
            ExtensionError::IoError(err) => write!(f, "IO error: {}", err),
            ExtensionError::ZedError(err) => write!(f, "Zed API error: {}", err),
            ExtensionError::AssetNotFound(name) => write!(f, "Asset not found: {}", name),
            ExtensionError::UnsupportedPlatform(platform) => {
                write!(f, "Unsupported platform: {}", platform)
            }
            ExtensionError::ReleaseNotFound(repo) => {
                write!(f, "Release not found for repo: {}", repo)
            }
            ExtensionError::DownloadFailed(url) => write!(f, "Download failed for URL: {}", url),
            ExtensionError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

// Implement std::error::Error for ExtensionError
impl std::error::Error for ExtensionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ExtensionError::SettingsParseError(err) => Some(err),
            ExtensionError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

// Implement From conversions for common error types
impl From<std::io::Error> for ExtensionError {
    fn from(err: std::io::Error) -> Self {
        ExtensionError::IoError(err)
    }
}

impl From<serde_json::Error> for ExtensionError {
    fn from(err: serde_json::Error) -> Self {
        ExtensionError::SettingsParseError(err)
    }
}

// Define a type alias for Result
type ExtensionResult<T> = std::result::Result<T, ExtensionError>;

#[cfg(test)]
mod tests;

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
    ) -> ExtensionResult<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        let release = zed::latest_github_release(
            REPO_NAME,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|e| ExtensionError::ReleaseNotFound(format!("Failed to get latest release for {}: {}", REPO_NAME, e)))?;

        let (platform, arch) = zed::current_platform();
        let arch_str = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X86 => "i386",
            zed::Architecture::X8664 => "x86_64",
            _ => return Err(ExtensionError::UnsupportedPlatform(format!("architecture {:?}", arch))),
        };
        let os_str = match platform {
            zed::Os::Mac => "Darwin",
            zed::Os::Linux => "Linux",
            zed::Os::Windows => "Windows",
            _ => return Err(ExtensionError::UnsupportedPlatform(format!("OS {:?}", platform))),
        };
        let ext_str = match platform {
            zed::Os::Mac | zed::Os::Linux => "tar.gz",
            zed::Os::Windows => "zip",
            _ => return Err(ExtensionError::UnsupportedPlatform(format!("OS for extension {:?}", platform))),
        };

        let asset_name = format!(
            "{BINARY_NAME}_{os}_{arch}.{ext}",
            os = os_str,
            arch = arch_str,
            ext = ext_str
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| ExtensionError::AssetNotFound(asset_name.clone()))?;

        let version_dir = format!("{BINARY_NAME}-{}", release.version);
        fs::create_dir_all(&version_dir)?;
        let binary_path = format!("{}/{}", version_dir, BINARY_NAME);

        // Download and set up the binary if it doesn't already exist at the path.
        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            let file_kind = match platform {
                zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                zed::Os::Windows => zed::DownloadedFileType::Zip,
                // This case should ideally be caught by the ext_str match above,
                // but as a safeguard or if logic changes:
                _ => return Err(ExtensionError::UnsupportedPlatform(format!("OS for file kind {:?}", platform))),
            };

            zed::download_file(&asset.download_url, &version_dir, file_kind)
                .map_err(|e| ExtensionError::DownloadFailed(format!("Failed to download {} from {}: {}",BINARY_NAME, asset.download_url, e)))?;

            zed::make_file_executable(&binary_path)
                .map_err(|e| ExtensionError::ZedError(format!("Failed to make file executable {}: {}", binary_path, e)))?;
        }

        // Cleanup old versions. This can run regardless of whether we just downloaded.
        let entries = fs::read_dir(".")?;
        for entry in entries {
            let entry = entry?;
            if entry.file_name().to_str() != Some(&version_dir) {
                // Ignoring result of remove_dir_all as it's a cleanup task
                fs::remove_dir_all(entry.path()).ok();
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
        info!("Buildkite MCP: context_server_command called");
        let settings_container = ContextServerSettings::for_project("mcp-server-buildkite", project)
            .map_err(|e| ExtensionError::ZedError(format!("Failed to get settings for project mcp-server-buildkite: {}", e)).to_string())?;

        let Some(settings_value) = settings_container.settings else {
            error!("Buildkite MCP: missing `buildkite_api_token` setting for mcp-server-buildkite");
            return Err(ExtensionError::ProjectOrSettingsMissing(
                "Buildkite API token is not configured. Please set it in the extension settings (mcp-server-buildkite).".to_string()
            ).to_string().into());
        };

        let settings: BuildkiteContextServerSettings = match serde_json::from_value(settings_value) {
            Ok(val) => val,
            Err(e) => {
                error!("Buildkite MCP: error parsing settings: {}", e);
                return Err(ExtensionError::SettingsParseError(e).to_string().into());
            }
        };
        info!("Buildkite MCP: parsed settings: {:?}", settings);

        let binary_path = self.context_server_binary_path(_context_server_id)
            .map_err(|e| e.to_string())?;
        info!("Buildkite MCP: launching binary at {}", binary_path);

        Ok(Command {
            command: binary_path,
            args: vec!["stdio".to_string()],
            env: vec![("BUILDKITE_API_TOKEN".into(), settings.buildkite_api_token)],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(BuildkiteContextServerSettings))
                .map_err(|e| ExtensionError::SettingsParseError(e).to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(BuildkiteMCPExtension);
