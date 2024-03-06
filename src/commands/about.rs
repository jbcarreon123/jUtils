use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Error;
use crate::{EmbedHelper, CONFIG};
use crate::types::Context;
use poise::serenity_prelude::{CreateEmbed, CreateActionRow, CreateButton};
use poise::serenity_prelude::CreateAllowedMentions as am;
use serenity::model::colour::Colour;

/// About the bot
#[poise::command(slash_command, prefix_command)]
pub async fn about(
    ctx: Context<'_>
) -> Result<(), Error> {
    _ = ctx.defer().await;
    let cu = ctx.http().get_current_user().await.expect("Expected a current user.");

    let modified = if CONFIG.jutils.modified {
        " modified "
    } else {
        " "
    };

    let avatar_url = match cu.avatar_url() {
        Some(a) => a,
        None => cu.default_avatar_url()
    };

    let c = CONFIG.about.color;

    let embed = CreateEmbed::primary()
        .title(format!("{}", cu.name))
        .description(CONFIG.about.description.clone())
        .thumbnail(avatar_url)
        .color(Colour::from_rgb(c[0], c[1], c[2]))
        .field("Hosted by", format!("<@{}>", CONFIG.about.bot_hoster), true)
        .field(format!("Using{}jUtils", modified), format!("version {}\nby jbcarreon123", env!("CARGO_PKG_VERSION")), true);

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