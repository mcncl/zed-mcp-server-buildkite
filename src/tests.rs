#[cfg(test)]
mod buildkite_mcp_extension_tests {
    // Items from the main library file (mcp_server_buildkite.rs)
    // are accessed via `crate::` because mcp_server_buildkite.rs is the lib root.
    use crate::{BuildkiteContextServerSettings, BuildkiteMCPExtension, ExtensionError};
    use zed_extension_api::{serde_json, Os, Architecture, GithubRelease, Asset, Project, ContextServerId};
    use std::path::Path;
    use std::collections::HashMap;


    #[test]
    fn test_settings_deserialization() {
        let json = r#"{"buildkite_api_token": "test-token"}"#;
        let settings: BuildkiteContextServerSettings =
            serde_json::from_str(json).expect("Failed to deserialize settings");

        assert_eq!(settings.buildkite_api_token, "test-token");
    }

    // Basic test to make sure the struct is defined correctly
    #[test]
    fn test_settings_struct() {
        let settings = BuildkiteContextServerSettings {
            buildkite_api_token: "api-token-value".to_string(),
        };

        assert_eq!(settings.buildkite_api_token, "api-token-value");
    }

    // Placeholder for more complex tests of context_server_binary_path
    #[test]
    fn test_context_server_binary_path_logic_placeholder() {
        // Testing context_server_binary_path thoroughly requires mocking:
        // - zed::latest_github_release
        // - zed::current_platform
        // - zed::download_file
        // - std::fs operations
        // This typically involves abstracting these dependencies behind traits
        // or using conditional compilation with mock implementations.
        // For now, this test serves as a placeholder.
        // The platform/architecture matching robustness was improved directly in the code
        // by returning ExtensionError::UnsupportedPlatform.
        assert!(true, "Placeholder for deeper context_server_binary_path tests");
    }
}
