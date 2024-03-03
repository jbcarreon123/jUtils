use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Error;
use crate::types::Context;

/// About the bot
#[poise::command(slash_command, prefix_command)]
pub async fn about(
    ctx: Context<'_>
) -> Result<(), Error> {
    let ping = ctx.ping().await;

    let response = format!("test");
    ctx.say(response).await?;
    Ok::<(), Error>(())
}