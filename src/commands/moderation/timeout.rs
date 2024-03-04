use std::default;
use std::error::Error;

use poise::serenity_prelude as serenity;
use crate::types::Context;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use poise::serenity_prelude::Permissions;
use poise::command;
use poise::serenity_prelude::*;
use poise::serenity_prelude::Guild;
use duration_string::DurationString;
use std::time::Duration;
use chrono::format::strftime;
use crate::utils::*;

/// Times out a user.
#[poise::command(
    slash_command,
    prefix_command,
    category="Moderation",
    required_permissions = "MODERATE_MEMBERS",
    default_member_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "MODERATE_MEMBERS | SEND_MESSAGES | EMBED_LINKS",
    guild_only,
    aliases("mute", "tm", "moderate", "unvoice")
)]
pub async fn timeout(
    ctx: Context<'_>,
    #[description="The user to time out."]
    mut user: Member,
    #[description="The timeout duration."]
    time: String,
    #[description="The timeout reason, if any."]
    reason: Option<String>
) -> Result<(), poise::serenity_prelude::Error> {
    _ = ctx.defer().await;

    let cu = ctx.http().get_current_user().await.expect("Expected a current user.");
    let rea: String = match reason {
        Some(str) => str,
        None => "No reason provided".to_owned()
    };
    let dur: Duration = DurationString::from_string(String::from(time)).unwrap().into();
    let guild = ctx.http().get_guild(ctx.guild_id().expect("Expected GuildId")).await.expect("Expected Guild");
    let embed: CreateEmbed =
    if &user.user == ctx.author() {
        let em = CreateEmbed::default()
            .title("Failed to time out user")
            .description("You cannot time out yourself.");
        em
    } else if compare_roles(ctx.serenity_context(), guild, user.user.id).await {
        let em = CreateEmbed::default()
            .title("Failed to time out user")
            .description(format!("{} can't time out users that has a higher role!", cu.name));
        em
    } else {
        match user.edit(ctx.http(), EditMember::new().disable_communication_until(duration_to_rfc3339(dur))).await {
            Ok(_) => {
                let em = CreateEmbed::default()
                    .title(format!("{} has been timed out", user.display_name()))
                    .description(rea)
                    .field("Duration", format_duration(dur), true)
                    .field("Timed out by", ctx.author().mention().to_string(), true);
                em
            }
            Err(E) => {
                let em = CreateEmbed::default()
                    .title("Failed to time out user")
                    .description(E.to_string());
                em
            }
        }
    };

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok(())
}