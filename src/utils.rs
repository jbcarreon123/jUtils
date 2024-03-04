use poise::serenity_prelude as serenity;
use serenity::builder::CreateEmbed;
use serenity::EmbedField;
use poise::serenity_prelude::Http;
use serenity::CurrentUser;
use poise::serenity_prelude::PartialGuild;
use poise::serenity_prelude::Mentionable;
use std::time::{Duration, UNIX_EPOCH};
use tokio::time::error::Elapsed;
use crate::config;
use poise::serenity_prelude::Context;
use poise::serenity_prelude::json::Value;
use poise::serenity_prelude::Guild;
use poise::serenity_prelude::UserId;
use poise::Command;
use chrono::DateTime;
use chrono::Utc;

pub async fn get_all_commands_as_embedfields<U, E>(
    ctx: poise::Context<'_, U, E>
) -> Result<Vec<Vec<(String, String, bool)>>, serenity::Error> {
    let config = config::load_config().expect("Expected the config to be found.");
    let mut commands = Vec::<&Command<U, E>>::new();
    for cmd in &ctx.framework().options().commands {
        commands
            .push(cmd);
    }

    let mut menu = Vec::<(String, String, bool)>::new();
    for command in commands {
        let desc: String = match &command.description {
            Some(string) => string.clone(),
            None => "No description".to_owned()
        };
        let pref = if ctx.prefix().starts_with("<") {
            config.discordbot.prefix.clone()
        } else {
            ctx.prefix().to_owned()
        };
        let space = if ctx.prefix().starts_with("<") {
            " "
        } else {
            ""
        };
        menu.push((
            format!("{}{}{}", pref, space, command.name),
            desc,
            true
        ))
    }

    Ok(chunk(menu, 25))
}

pub fn chunk<T: Clone>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    vec.into_iter()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>()
}

pub fn format_duration(duration: Duration) -> String {
    let mut duration_secs = duration.as_secs();
    let days = duration_secs / (24 * 3600);
    duration_secs %= 24 * 3600;
    let hours = duration_secs / 3600;
    duration_secs %= 3600;
    let minutes = duration_secs / 60;
    let seconds = duration_secs % 60;

    let mut result = String::new();
    if days > 0 {
        result.push_str(&format!("{:02}:", days));
    }
    if hours > 0 || days > 0 {
        result.push_str(&format!("{:02}:", hours));
    }
    result.push_str(&format!("{:02}:{:02}", minutes, seconds));

    result
}

pub fn duration_to_rfc3339(duration: Duration) -> String {
    let now: DateTime<Utc> = Utc::now();
    let datetime = now + duration;
    datetime.to_rfc3339()
}

pub async fn compare_roles(ctx: &Context, guild: PartialGuild, user_id: UserId) -> bool {
    let cu = ctx.http.get_current_user().await.expect("Expected a current user.");
    let bot_member = match guild.member(ctx.http.clone(), cu.id).await {
        Ok(member) => member,
        Err(E) => return false
    };
    let user_member = match guild.member(ctx.http.clone(), user_id).await {
        Ok(member) => member,
        Err(E) => return false,
    };

    let bhighest_role = bot_member.highest_role_info(ctx.cache.clone()).expect("Expected roles");
    let uhighest_role = user_member.highest_role_info(ctx.cache.clone()).expect("Expected roles");

    if bhighest_role.1 < uhighest_role.1 {
        return true
    }
    false
}