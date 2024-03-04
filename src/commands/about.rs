use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Error;
use crate::types::Context;
use poise::serenity_prelude::{CreateEmbed, CreateActionRow, CreateButton};
use poise::serenity_prelude::CreateAllowedMentions as am;

/// About the bot
#[poise::command(slash_command, prefix_command)]
pub async fn about(
    ctx: Context<'_>
) -> Result<(), Error> {
    let cu = ctx.http().get_current_user().await.expect("Expected a current user.");

    let embed = CreateEmbed::default()
        .title(format!("{}", cu.name))
        .description("jUtils is a multi-purpose bot that is written in Rust!")
        .field("Written by", "<@908002848967626842>", true);

    let mut components = Vec::<CreateActionRow>::new();
    components.push(CreateActionRow::Buttons([ CreateButton::new_link("https://github.com/jbcarreon123/jUtils").label("GitHub Repo") ].to_vec()));

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .components(components)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}