
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct DiscordBotConfig {
    pub token: String,
    pub prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct FontConfig {
    pub inter: String,
    pub inter_semibold: String,
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
pub struct LevelingCardConfig {
    /// When to allow animated leveling cards.
    /// <div class="warning">Enabling this can be computationally expensive.</div>
    pub allow_animated: bool,
    /// Users to only render animated cards on.
    /// Omit this if you want to enable it to everybody.
    pub allowed_users_animated: Vec<String>
}

/// The jUtils configuration system.
/// 
/// Provides the configs for things that the instance owner
/// should only configure. All of these requires a bot restart.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub discordbot: DiscordBotConfig,
    pub database: DatabaseConfig,
    pub font: FontConfig,
    pub github: GitHubConfig,
    pub rsi: RocScamIndexConfig,
    pub motd: MOTDConfig,
    pub about: AboutConfig,
    pub jutils: JUtilsConfig,
    pub colors: ColorsConfig,
    pub emoji: EmojiConfig,
    pub lvlcard: LevelingCardConfig
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(".cfg/CONFIG.toml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let config: Config = toml::from_str(&content)?;

    Ok(config)
}
