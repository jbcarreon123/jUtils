

use poise::serenity_prelude::Error;


use crate::types::Context;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;

/// Get information of an invite.
#[poise::command(slash_command, prefix_command, category="Utilities", aliases("pong"))]
pub async fn invite(
    ctx: Context<'_>,
    mut invite: String
) -> Result<(), Error> {
    _ = ctx.defer().await;
    invite = invite.trim_start_matches("https://").trim_start_matches("discord.gg/").trim_start_matches("discord.com/invite/").to_owned();
    let embed = match ctx.http().get_invite(invite.as_str(), true, true, None).await {
        Ok(inv) => {
            let mut embed = CreateEmbed::primary()
                .title(format!("Invite `{}`", inv.code))
                .field("Channel", format!("{}\n{}", inv.channel.name, inv.channel.id), true);
            if let Some(inviter) = inv.inviter {
                embed = embed.clone().field("Inviter", format!("{}\n{}", inviter.name, inviter.id), true);
            }
            if let Some(expires_at) = inv.expires_at {
                embed = embed.clone().field("Expires at", format!("<t:{}>", expires_at.timestamp()), true);
            }
            if let Some(approx_mem_count) = inv.approximate_member_count {
                embed = embed.clone().field("Approx Member Count", format!("{} members", approx_mem_count), true);
            }
            if let Some(approx_pre_count) = inv.approximate_presence_count {
                embed = embed.clone().field("Approx Online Count", format!("{} members", approx_pre_count), true);
            }
            if let Some(guild) = inv.guild {
                embed = embed.clone().description(format!("**{}**\n{}", guild.name, guild.description.unwrap_or("No description".to_owned())));
                if let Some(icon) = guild.icon {
                    embed = embed.clone().thumbnail(format!("https://cdn.discordapp.com/icons/{}/{}.png", guild.id, icon.to_string()));
                }
                embed = embed.clone().field("Features", guild.features.join(", "), false)
            }
            
            embed
        }
        Err(why) => {
            CreateEmbed::error()
                .title("Cannot get invite!")
                .description(format!("{}", why))
        }
    };
    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}