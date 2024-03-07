use poise::serenity_prelude::Error;
use crate::types::Context;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;

#[poise::command(
    prefix_command,
    hide_in_help,
    identifying_name = "jUtils.internal.ee.roc"
)]
pub async fn roc(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default()
        .content("https://tenor.com/view/the-rock-dwayne-johnson-the-rock-eating-rock-gif-13935567355130525471")
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}

#[poise::command(
    prefix_command,
    hide_in_help,
    identifying_name = "jUtils.internal.ee.gowthr"
)]
pub async fn gowthr(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default()
        .content("Gowthr was here...")
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}