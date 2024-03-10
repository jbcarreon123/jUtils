use std::{io::ErrorKind};

use bson::{DateTime, Document};
use poise::serenity_prelude::CreateAllowedMentions as am;
use chrono::Utc;
use mongodb::{
    bson::doc, error::Error, options::{FindOneOptions, FindOptions, UpdateOptions}, Client, Collection, Database
};
use serde::{Deserialize, Serialize};
use serenity::{all::{CreateEmbed, User}, futures::StreamExt};
use crate::{guild_config::config::Category, utils::generate_id, EmbedHelper, CONFIG};

pub async fn load_db() -> Database {
    let c = Client::with_uri_str(CONFIG.database.connection_string.clone()).await.expect("Can't connect to the MongoDB database!");
    c.database(&CONFIG.database.db)
}

// Warns System

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Warn {
    pub id: String,
    pub issuer: String,
    pub user: String,
    pub reason: String,
    pub duration: DateTime,
    pub resolved: bool
}

impl Warn {
    pub fn new(issuer: String, user: String, reason: String, dur: chrono::DateTime<Utc>) -> Self {
        let resolved = false;
        let id = generate_id(6);
        let duration = DateTime::from_chrono::<Utc>(dur);
        Warn {
            id,
            issuer,
            user,
            reason,
            duration,
            resolved
        }
    }
}

pub trait WarnEmbedHelper {
    fn to_embed(&self, user: User) -> CreateEmbed;
}

impl WarnEmbedHelper for Vec<Warn> {
    fn to_embed(&self, user: User) -> CreateEmbed {
        let mut embed = CreateEmbed::default()
            .title(format!("{}'s warns", user.name));
        for warn in self {
            let res = if warn.resolved {
                "Resolved "
            } else {
                ""
            };
            embed = embed.clone().field(
                format!("{}Warn {}", res, warn.id),
                format!("{}\n\nDuration: {}\nissued by <@{}>", warn.reason, warn.duration, warn.issuer),
                true);
        }
        embed
    }
}

pub async fn get_warn(
    id: String
) -> Result<Option<Warn>, mongodb::error::Error> {
    let collection = get_warns().await;
    let filter = doc! { "id": id };
    let options = FindOneOptions::default();

    match collection.find_one(filter, options).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

pub async fn get_warns() -> Collection<Warn> {
    let db = load_db().await;
    db.collection::<Warn>("warns")
}

pub async fn get_warnings_by_user(
    user_id: String
) -> Result<Vec<Warn>, mongodb::error::Error> {
    let collection = get_warns().await;
    let filter = doc! { "user": user_id };
    let options = FindOptions::default();
    let mut cursor = match collection.find(filter, options).await {
        Ok(res) => res,
        Err(e) => return Err(e.into())
    };

    let mut results: Vec<Warn> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(warn) => {
                results.push(warn);
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(results)
}

pub async fn resolve_warn(id: String) -> Result<(), mongodb::error::Error> {
    let collection = get_warns().await;
    let filter = doc! { "id": id };
    let update_doc = doc! { "$set": { "resolved": true } };
    let options = UpdateOptions::default();

    match collection.update_one(filter, update_doc, options).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn create_warn(issuer: String, user: String, reason: String, duration: chrono::DateTime<Utc>) -> Result<String, mongodb::error::Error> {
    let collection = get_warns().await;
    let nw = Warn::new(
        issuer,
        user,
        reason,
        duration
    );
    _ = collection.insert_one(
        nw.clone(),
        None
    ).await;

    Ok(nw.id)
}

// Guild Config System

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Module {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Moderation {
    pub enabled: bool,
    pub warns: WarnAction,
    pub nameban: NameBan,
    pub automodhook: AutoModHook,
    pub reports: ReportAction
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WarnAction {
    pub warn_count: u32,
    pub action: Action,
    pub duration: Option<String>, // Optional for BAN
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NameBan {
    pub enabled: bool,
    pub banned_names: Vec<String>,
    pub action: Action,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoModHook {
    pub enabled: bool,
    pub rules: Vec<AutoModRule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoModRule {
    pub rule_name: String,
    pub action: Action,
    pub duration: Option<String>, // Optional for BAN
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportAction {
    pub enabled: bool,
    pub report_channel: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WikiEntry {
    pub name: String,
    pub api_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Action {
    MUTE,
    KICK,
    BAN,
    WARN,
}

impl Action {
    pub fn to_string(&self) -> &str {
        match self {
            Action::BAN => "Ban user",
            Action::KICK => "Kick user",
            Action::MUTE => "Mute user",
            Action::WARN => "Warn user"
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CountingMode {
    DOWN,
    UP,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub name: String,
    pub api_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildConfig {
    pub guild_id: String,
    pub unconfigured: bool,
    pub modules: Modules,
}

impl GuildConfig {
    pub fn get_status_by_category(&self, cat: Category) -> bool {
        match cat {
            Category::Tickets => self.modules.tickets.enabled,
            Category::Counting => self.modules.counting.enabled,
            Category::Leveling => self.modules.leveling.enabled,
            Category::Moderation => self.modules.moderation.enabled,
            Category::ModerationAutoModHook => self.modules.moderation.automodhook.enabled,
            Category::ModerationNameBan => self.modules.moderation.nameban.enabled,
            Category::ModerationReports => self.modules.moderation.reports.enabled,
            Category::ServerQoL => self.modules.serverqol.enabled,
            Category::ServerStats => true,
            Category::Starboard => self.modules.starboard.enabled,
            Category::Tagging => self.modules.tagging.enabled,
            Category::Utilities => self.modules.utilities.enabled.discord_based,
            Category::UtilitiesWiki => self.modules.utilities.wiki.enabled,
            _ => false
        }
    }

    pub fn new(guild_id: String, modules: Modules) -> GuildConfig {
        GuildConfig {
            guild_id,
            unconfigured: false,
            modules
        }
    }

    pub fn default(guild_id: String) -> GuildConfig {
        GuildConfig {
            guild_id,
            unconfigured: true,
            modules: Modules {
                moderation: Moderation {
                    enabled: false,
                    warns: WarnAction {
                        warn_count: 0,
                        action: Action::MUTE,
                        duration: None,
                        reason: "".to_owned()
                    },
                    nameban: NameBan {
                        enabled: false,
                        action: Action::KICK,
                        banned_names: Vec::<String>::new(),
                        reason: "".to_owned()
                    },
                    automodhook: AutoModHook {
                        enabled: false,
                        rules: Vec::<AutoModRule>::new()
                    },
                    reports: ReportAction {
                        enabled: false,
                        report_channel: "0".to_owned()
                    }
                },
                serverqol: Module {
                    enabled: false
                },
                utilities: Utilities {
                    enabled: UtilityEnabled {
                        package: false,
                        discord_based: false,
                        github: false,
                        conversion: false,
                        trivia: false,
                        photos: false
                    },
                    wiki: Wiki {
                        enabled: false,
                        entries: Vec::<Entry>::new()
                    }
                },
                leveling: Leveling {
                    enabled: false,
                    multiplier: 1.0,
                    blacklisted_channels: Vec::<String>::new()
                },
                starboard: Starboard {
                    enabled: false,
                    threshold: 1,
                    channel: "0".to_owned(),
                    star: ":star:".to_owned()
                },
                counting: Counting {
                    enabled: false,
                    channel: "0".to_owned(),
                    mode: CountingMode::UP
                },
                tickets: Module {
                    enabled: false
                },
                tagging: Module {
                    enabled: false
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Modules {
    pub moderation: Moderation,
    pub serverqol: Module,
    pub utilities: Utilities,
    pub leveling: Leveling,
    pub starboard: Starboard,
    pub counting: Counting,
    pub tickets: Module,
    pub tagging: Module,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Utilities {
    pub enabled: UtilityEnabled,
    pub wiki: Wiki,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UtilityEnabled {
    pub package: bool,
    pub discord_based: bool,
    pub github: bool,
    pub conversion: bool,
    pub trivia: bool,
    pub photos: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wiki {
    pub enabled: bool,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Leveling {
    pub enabled: bool,
    pub multiplier: f64,
    pub blacklisted_channels: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Starboard {
    pub enabled: bool,
    pub threshold: u32,
    pub channel: String,
    pub star: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Counting {
    pub enabled: bool,
    pub channel: String,
    pub mode: CountingMode,
}

pub async fn get_guild_config(
    guild_id: String
) -> Result<GuildConfig, mongodb::error::Error> {
    let db = load_db().await;
    let collection = db.collection::<GuildConfig>("guild_config");
    let filter = doc! { "guild_id": guild_id };
    let options = FindOptions::default();
    let mut cursor = match collection.find(filter, options).await {
        Ok(res) => res,
        Err(e) => return Err(e.into())
    };


    let mut config: Option<GuildConfig> = None;

    while let Some(result) = cursor.next().await {
        match result {
            Ok(conf) => {
                config = Some(conf);
                break;
            }
            Err(e) => return Err(e.into()),
        }
    }

    config.ok_or_else(|| Error::from(ErrorKind::NotFound))
}

pub async fn set_guild_config(
    guild_id: String,
    modules: Modules
) -> Result<(), mongodb::error::Error> {
    let db = load_db().await;
    let collection = db.collection::<GuildConfig>("guild_config");
    let filter = doc! { "guild_id": guild_id.clone() };

    let options = UpdateOptions::builder()
        .upsert(true)
        .build();

    let mut config = GuildConfig::new(guild_id, modules);
    config.unconfigured = false;
    let bson_conf: Document = bson::to_document(&config)?;

    match collection
        .update_one(filter, bson_conf, options)
        .await {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
}

pub async fn create_new_config(guild_id: String) -> Result<(), mongodb::error::Error> {
    let db = load_db().await;
    let collection = db.collection::<GuildConfig>("guild_config");
    let nw = GuildConfig::default(guild_id);
    _ = collection.insert_one(
        nw.clone(),
        None
    ).await;

    Ok(())
}

// TODO

pub async fn is_guild_configured<U, E>(ctx: poise::Context<'_, U, E>) -> Result<bool, serenity::Error> {
    if ctx.guild().is_none() {
        return Ok(true)
    }

    let guild_id: String = ctx.guild_id().unwrap().to_string();
    let conf = match get_guild_config(guild_id).await {
        Ok(e) => e,
        Err(_) => {
            return Err(serenity::Error::Other("Cannot get guild config"))
        }
    };

    if conf.unconfigured {
        let embed = CreateEmbed::error()
            .title("This server is unconfigured!")
            .description("Please run /config to fix this!");

            _ = ctx.send(poise::CreateReply::default()
                .embed(embed)
                .reply(true)
                .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
            ).await;
    }

    Ok(!conf.unconfigured)
}