use poise::serenity_prelude::Error;
use crate::database::get_guild_config;
use crate::types::Context;
use poise::serenity_prelude::CreateAllowedMentions as am;

#[poise::command(
    prefix_command,
    category="GuildConfig",
    required_permissions = "MANAGE_GUILD",
    default_member_permissions = "MANAGE_GUILD",
    required_bot_permissions = "MANAGE_GUILD | SEND_MESSAGES | EMBED_LINKS",
    guild_only,
    identifying_name = "jUtils.config.guild.to",
    subcommands("toml", "json"),
    subcommand_required
)]
pub async fn config_to(
    _ctx: Context<'_>,
) -> Result<(), Error> {
    Ok(())
}

/// Gets the config JSON for this guild. For debugging purposes only.
#[poise::command(
    prefix_command
)]
pub async fn json(
    ctx: Context<'_>,
) -> Result<(), Error> {
    _ = ctx.defer().await;
    let conf = get_guild_config(ctx.guild_id().unwrap().to_string()).await.expect("Expected GuildConfig");
    ctx.send(poise::CreateReply::default()
        .content(format!("```json\n{}\n```", serde_json::to_string_pretty(&conf).expect("Failed to serialize to JSON")))
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}

/// Gets the config TOML for this guild. For debugging purposes only.
#[poise::command(
    prefix_command
)]
pub async fn toml(
    ctx: Context<'_>,
) -> Result<(), Error> {
    _ = ctx.defer().await;
    let conf = get_guild_config(ctx.guild_id().unwrap().to_string()).await.expect("Expected GuildConfig");
    ctx.send(poise::CreateReply::default()
        .content(format!("```toml\n{}\n```", toml::to_string_pretty(&conf).expect("Failed to serialize to JSON")))
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}