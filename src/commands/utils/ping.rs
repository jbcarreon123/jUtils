use std::time::Instant;

use poise::serenity_prelude::Error;
use crate::database::load_db;
use crate::types::Context;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;

/// Pings the bot.
#[poise::command(slash_command, prefix_command, category="Utilities", aliases("pong"))]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error> {
    _ = ctx.defer().await;
    let ping = ctx.ping().await;

    let start = Instant::now();
    _ = load_db().await;
    let elapsed = start.elapsed();

    let embed = CreateEmbed::success()
        .title("Pong! :ping_pong:")
        .field("API Latency", format!("{:?}", ping), true)
        .field("Database Latency", format!("{:?}", elapsed), true);
    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}