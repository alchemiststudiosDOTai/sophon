#[derive(Debug, Clone)]
pub struct ExaConfig {
    pub api_key: String,
    pub base_url: String,
}

impl ExaConfig {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let api_key = std::env::var("EXA_API_KEY")?;
        if api_key.trim().is_empty() {
            return Err(std::env::VarError::NotPresent);
        }

        Ok(Self {
            api_key,
            base_url: "https://api.exa.ai".to_string(),
        })
    }
}
