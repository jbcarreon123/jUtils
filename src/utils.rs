use poise::serenity_prelude as serenity;
use poise::serenity_prelude::PartialGuild;
use std::time::Duration;
use crate::config;
use poise::serenity_prelude::Context;
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
        let permitted = if ctx.guild_id().is_none() {
            if command.dm_only {
                true
            } else {
                false
            }
        } else {
            let g = ctx.guild().expect("Expected guild").clone();
            let gusr = match g.member(ctx.http(), ctx.author().clone().id).await {
                Ok(u) => u,
                _error => continue
            };
            let perms = gusr.permissions(ctx.cache()).expect("Expected guild member permissions").clone();
            perms.contains(command.required_permissions)
        };

        if command.hide_in_help || !permitted {
            continue
        }

        let desc: String = match &command.description {
            Some(string) => string.clone(),
            None => "No description".to_owned()
        };
        let cat: String = match &command.category {
            Some(str) => format!("Category: {}\n", str),
            None => "".to_owned()
        };
        let pref = if ctx.prefix().starts_with("<@") {
            config.discordbot.prefix.clone()
        } else {
            ctx.prefix().to_owned()
        };
        menu.push((
            format!("{}{}", pref, command.name),
            format!("{}{}", cat, desc),
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
        Err(_) => return false
    };
    let user_member = match guild.member(ctx.http.clone(), user_id).await {
        Ok(member) => member,
        Err(_) => return false,
    };

    let bhighest_role = bot_member.highest_role_info(ctx.cache.clone()).expect("Expected roles");
    let uhighest_role = user_member.highest_role_info(ctx.cache.clone()).expect("Expected roles");

    if bhighest_role.1 < uhighest_role.1 {
        return true
    }
    false
}