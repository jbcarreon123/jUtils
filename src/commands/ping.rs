use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Error;
use crate::types::Context;

#[poise::command(slash_command, prefix_command)]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error> {
    let ping = ctx.ping().await;

    let response = format!("Pong! Latency: {ping:?}");
    ctx.say(response).await?;
    Ok::<(), Error>(())
}