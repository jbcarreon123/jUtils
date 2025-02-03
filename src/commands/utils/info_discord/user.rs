use itertools::Itertools;
use poise::serenity_prelude::Error;
use serenity::all::Member;
use serenity::all::Mentionable;
use crate::types::Context;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;

/// Get information of a specific user.
#[poise::command(
    slash_command,
    prefix_command,
    category="Utilities",
    aliases("userinfo", "whois"),
    guild_only
)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "The user you want to get the information to. Defaults to yourself."]
    member: Option<Member>,
) -> Result<(), Error> {
    _ = ctx.defer().await;
    let guild = ctx.guild().expect("Expected Guild").clone();
    let member = member.unwrap_or(ctx.author_member().await.expect("Expected Member").into_owned());
    let user = member.user;
    let embed = CreateEmbed::default()
        .title(format!("{} ({}, `{}`)", user.display_name(), user.tag(), user.id))
        .color(user.accent_colour.unwrap_or_default().0)
        .thumbnail(user.avatar_url().unwrap_or_default())
        .field("Bot", user.bot.to_string(), true)
        .field("System", user.system.to_string(), true)
        .field("Mention", user.mention().to_string(), true)
        .field("Created at", format!("<t:{}>", user.created_at().timestamp()), true)
        .field("Joined at", format!("<t:{}>", member.joined_at.unwrap().timestamp()), true)
        .field("Boosting since", member.premium_since.map(|t| format!("<t:{}>", t.timestamp())).unwrap_or_else(|| "Not boosting".to_string()), true)
        .field("Guild Flags", member.flags.iter_names().map(|f| f.0).join(", "), false)
        .field("Public Flags", user.flags.iter_names().map(|f| f.0).join(", "), false)
        .field("Roles", member.roles.iter().map(|r| r.mention().to_string()).join(", "), false);
    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}