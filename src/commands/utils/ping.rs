use poise::serenity_prelude::Error;
use crate::types::Context;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;

/// Pings the bot.
#[poise::command(slash_command, prefix_command, category="Utilities")]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error> {
    _ = ctx.defer().await;
    let ping = ctx.ping().await;

    let embed = CreateEmbed::success()
        .title("Pong! :ping_pong:")
        .field("API Latency", format!("{:?}", ping), false);
    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}