use std::borrow::Cow;
use std::time::Instant;
use regex::Regex;
use poise::serenity_prelude::Error;
use crate::types::Context;
use crate::utils::utils::edit_channel_name;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;

/// Mass edits channels.
#[poise::command(
    slash_command,
    category="Utilities",
    aliases("meditchnl", "masseditc", "mec"),
    required_permissions = "MANAGE_CHANNELS | MANAGE_GUILD",
    default_member_permissions = "MANAGE_CHANNELS | MANAGE_GUILD",
    required_bot_permissions = "MANAGE_CHANNELS | MANAGE_GUILD | SEND_MESSAGES | EMBED_LINKS",
)]
pub async fn massedit_channel(
    ctx: Context<'_>,
    find: String,
    replace: String
) -> Result<(), Error> {
    _ = ctx.defer().await;
    let start = Instant::now();
    let channels = ctx
        .serenity_context()
        .http
        .clone()
        .get_channels(ctx.guild_id().unwrap().clone()).await;

    let re = Regex::new(&find).unwrap();
    let embed = if let Ok(e) = channels {
        for channel in e {
            let rep = replace.clone();
            let replaced_name = re.replace_all(&channel.name, &rep);
            let new_name: String = match replaced_name {
                Cow::Borrowed(s) => s.to_owned(),
                Cow::Owned(s) => s,
            };
            edit_channel_name(channel.to_owned(), ctx.http(), new_name).await;
        }

        let elapsed = start.elapsed();
        CreateEmbed::success()
            .title("Mass editing complete")
            .description(format!("Process took {:?}", elapsed))
    } else {
        CreateEmbed::error()
            .title("Cannot find guild!")
    };

    _ = ctx.send(
        poise::CreateReply::default()
            .embed(embed)
            .reply(true)
            .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await;

    Ok(())
}
