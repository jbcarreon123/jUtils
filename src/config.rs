use toml::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::ptr::null;

#[derive(Debug, Deserialize)]
pub struct DiscordBotConfig {
    pub token: String,
    pub prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubConfig {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct RocScamIndexConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct MOTDConfig {
    pub motd_timeout: u64,
    pub include_help_prefix: bool,
    pub motd_strings: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub discordbot: DiscordBotConfig,
    pub database: DatabaseConfig,
    pub github: GitHubConfig,
    pub rsi: RocScamIndexConfig,
    pub motd: MOTDConfig
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open("config.toml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let config: Config = toml::from_str(&content)?;

    Ok(config)
}