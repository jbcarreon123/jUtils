use std::error::Error;

use chrono::Duration;
use duration_string::DurationString;
use poise::serenity_prelude::Error as PoiseError;
use serde_json::json;
use serenity::all::Member;
use serenity::all::Mentionable;
use crate::database::create_warn;
use crate::database::get_warnings_by_user;
use crate::database::load_db;
use crate::database::Warn;
use crate::database::WarnEmbedHelper;
use crate::types::Context;
use crate::utils::duration_to_datetime;
use crate::utils::format_duration;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;
use serde_json::Serializer;

/// Get warns
#[poise::command(
    slash_command,
    prefix_command,
    category="Moderation",
    guild_only,
    aliases("strikes"),
    broadcast_typing,
    track_deletion,
    identifying_name = "jUtils.moderation.warns.list_warns"
)]
pub async fn warns(
    ctx: Context<'_>,
    #[description="The user you want to get the warns to. Requires MODERATE_MEMBERS."]
    user: Option<Member>
) -> Result<(), PoiseError> {
    _ = ctx.defer().await;
    let auth = ctx.author_member().await.expect("Expected member");
    let perms = auth.permissions(ctx.cache()).expect("Expected permissions");

    let embed = if user.is_some() && perms.moderate_members() {
        let u = user.unwrap();
        match get_warnings_by_user(u.user.id.into()).await {
            Ok(w) => w.to_embed(u.user),
            Err(_) => {
                CreateEmbed::error()
                    .title("An error occured")
                    .description("There is no warns or the user does not exist.")
            }
        }
    } else {
        match get_warnings_by_user(ctx.author().id.into()).await {
            Ok(w) => w.to_embed(ctx.author().to_owned()),
            Err(_) => {
                CreateEmbed::error()
                    .title("An error occured")
                    .description("There is no warns or the user does not exist.")
            }
        }
    };

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), PoiseError>(())
}