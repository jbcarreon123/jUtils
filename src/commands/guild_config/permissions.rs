use poise::serenity_prelude::Error;
use crate::types::Context;
use crate::utils::*;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;

/// Shows permissions of jUtils in the guild.
#[poise::command(
    slash_command,
    prefix_command,
    category="GuildConfig",
    required_permissions = "MANAGE_GUILD",
    default_member_permissions = "MANAGE_GUILD",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    guild_only,
    aliases("perms"),
    identifying_name = "jUtils.config.permissions"
)]
pub async fn permissions(
    ctx: Context<'_>
) -> Result<(), Error> {
    _ = ctx.defer().await;
    let embedflds = get_perms_as_embedfields(ctx.clone()).await.expect("Expected permissions");

    let embed = CreateEmbed::default()
        .title("Permissions of the bot on this server")
        .fields(embedflds.clone());
    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;

    Ok(())
}