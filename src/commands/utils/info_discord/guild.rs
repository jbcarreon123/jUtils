use poise::serenity_prelude::Error;
use crate::types::Context;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;

/// Get information of a guild.
#[poise::command(
    slash_command,
    prefix_command,
    category="Utilities",
    aliases("serverinfo, guildinfo, server"),
    required_permissions = "MODERATE_MEMBERS",
    default_member_permissions = "MODERATE_MEMBERS",
    guild_only)]
pub async fn guild(
    ctx: Context<'_>
) -> Result<(), Error> {
    _ = ctx.defer().await;
    
    let embed = match ctx.guild() {
        Some(guild) => {
            let mut emb = CreateEmbed::primary()
                .title(format!("{} (`{}`)", guild.name, guild.id))
                .description(guild.description.clone().unwrap_or("No description".to_owned()))
                .field("Owner", format!("<@{}>\n`{}`", guild.owner_id, guild.owner_id), true)
                .field("Features", guild.features.join(", "), false);

            if let Some(icon) = guild.icon_url() {
                emb = emb.clone().thumbnail(icon);
            }
            if let Some(banner) = guild.banner_url() {
                emb = emb.clone().image(banner)
            } else if let Some(splash) = guild.splash_url() {
                emb = emb.clone().image(splash)
            } else if let Some(splash) = guild.discovery_splash {
                emb = emb.clone().image(format!("https://cdn.discordapp.com/discovery-splashes/{}/{}.png", guild.id, splash.to_string()))
            }

            emb
        }
        None => {
            CreateEmbed::error()
                .title("This isn't a guild!")
        }
    };

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}