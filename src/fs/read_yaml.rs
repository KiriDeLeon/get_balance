use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct WalletData {
    pub wallets: Vec<String>,
}

pub fn load_config(path: &str) -> Result<WalletData> {
    let contents = fs::read_to_string(path).with_context(|| format!("Err file path: {}", path))?;
    let config: WalletData = serde_yaml::from_str(&contents).context("YAML parsing err")?;
    Ok(config)
}
