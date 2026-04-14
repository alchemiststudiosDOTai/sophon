#[derive(Debug, Clone)]
pub struct BraveConfig {
    pub api_key: String,
    pub base_url: String,
}

impl BraveConfig {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let api_key = std::env::var("BRAVE_API_KEY")?;
        Ok(Self {
            api_key,
            base_url: "https://api.search.brave.com/res/v1".to_string(),
        })
    }
}
