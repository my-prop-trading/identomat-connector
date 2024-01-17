#[derive(Clone, Debug)]
pub struct IdentomatConfig {
    pub rest_api_host: String,
}

impl Default for IdentomatConfig {
    fn default() -> Self {
        Self {
            rest_api_host: "https://widget.identomat.com".into(),
        }
    }
}

impl IdentomatConfig {
    pub fn test_env() -> Self {
        Self {
            rest_api_host: "https://widget.identomat.com".into(),
        }
    }
}
