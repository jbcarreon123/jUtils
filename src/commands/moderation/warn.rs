use std::error::Error;

use chrono::Duration;
use duration_string::DurationString;
use poise::serenity_prelude::Error as PoiseError;
use serde_json::json;
use serenity::all::Member;
use serenity::all::Mentionable;
use crate::database::create_warn;
use crate::database::load_db;
use crate::database::Warn;
use crate::types::Context;
use crate::utils::duration_to_datetime;
use crate::utils::format_duration;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;
use serde_json::Serializer;

/// Warn a user
#[poise::command(
    slash_command,
    prefix_command,
    category="Moderation",
    required_permissions = "MODERATE_MEMBERS",
    default_member_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "MODERATE_MEMBERS | SEND_MESSAGES | EMBED_LINKS",
    guild_only,
    aliases("strike")
)]
pub async fn warn(
    ctx: Context<'_>,
    #[description="The user to warn."]
    user: Member,
    #[description="The warn duration."]
    duration: String,
    #[description="The warn reason, if any."]
    reason: Option<String>
) -> Result<(), PoiseError> {
    _ = ctx.defer().await;
    let rea: String = match reason {
        Some(str) => str,
        None => "No reason provided".to_owned()
    };
    let dur: std::time::Duration = DurationString::from_string(String::from(duration)).unwrap().into();
    let dt = duration_to_datetime(dur);
    let embed = match create_warn(ctx.author().id.into(), user.user.id.into(), rea.clone(), dt).await {
        Ok(id) => {
            CreateEmbed::success()
                .title(format!("{} has been warned", user.user.name))
                .description(rea)
                .field("Warn ID", id, true)
                .field("Duration", format_duration(dur), true)
                .field("Warned by", ctx.author().mention().to_string(), true)
        }
        Err(err) => {
            CreateEmbed::error()
                .title("Failed to warn user")
                .description(err.to_string())
        }
    };

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), PoiseError>(())
}