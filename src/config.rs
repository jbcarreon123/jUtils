use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct DiscordBotConfig {
    pub token: String,
    pub prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub db: String
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
pub struct AboutConfig {
    pub bot_hoster: u64,
    pub description: String,
    pub color: [u8; 3]
}

#[derive(Debug, Deserialize)]
pub struct JUtilsConfig {
    pub modified: bool
}

#[derive(Debug, Deserialize)]
pub struct ColorsConfig {
    pub primary: [u8; 3],
    pub secondary: [u8; 3],
    pub success: [u8; 3],
    pub error: [u8; 3],
    pub invis: [u8; 3]
}

#[derive(Debug, Deserialize)]
pub struct EmojiConfig {
    pub check_box: String,
    pub cross_box: String
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub discordbot: DiscordBotConfig,
    pub database: DatabaseConfig,
    pub github: GitHubConfig,
    pub rsi: RocScamIndexConfig,
    pub motd: MOTDConfig,
    pub about: AboutConfig,
    pub jutils: JUtilsConfig,
    pub colors: ColorsConfig,
    pub emoji: EmojiConfig
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open("CONFIG.toml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let config: Config = toml::from_str(&content)?;

    Ok(config)
}
