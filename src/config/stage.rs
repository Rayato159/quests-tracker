use anyhow::Result;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    #[default]
    Local,
    Development,
    Production,
}

impl Stage {
    pub fn try_from(stage: &str) -> Result<Self> {
        match stage.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            "local" => Ok(Self::Local),
            _ => Err(anyhow::anyhow!("Invalid stage")),
        }
    }
}
