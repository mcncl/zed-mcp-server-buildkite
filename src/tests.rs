#[cfg(test)]
mod tests {
    use crate::BuildkiteContextServerSettings;
    use zed_extension_api::serde_json;

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
}
