use poise::serenity_prelude::Error;
use crate::types::Context;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;

/// Configures the bot on the server.
#[poise::command(
    slash_command,
    prefix_command,
    category="GuildConfig",
    required_permissions = "MANAGE_GUILD",
    default_member_permissions = "MANAGE_GUILD",
    required_bot_permissions = "MANAGE_GUILD | SEND_MESSAGES | EMBED_LINKS",
    guild_only,
    aliases("guildconfig", "gcfg", "cfg", "serverconfig", "srvcfg"),
    identifying_name = "jUtils.config.guild"
)]
pub async fn config(
    ctx: Context<'_>
) -> Result<(), Error> {
    _ = ctx.defer().await;

    let embed = CreateEmbed::success()
        .title(format!("jUtils Config for {}", ctx.guild().unwrap().name));
    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}

pub async fn config_btn(id: String) {

}