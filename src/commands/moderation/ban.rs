use crate::types::Context;
use crate::EmbedHelper;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use poise::serenity_prelude::*;
use crate::utils::*;

/// Bans a user.
#[poise::command(
    slash_command,
    prefix_command,
    category="Moderation",
    required_permissions = "BAN_MEMBERS",
    default_member_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS | SEND_MESSAGES | EMBED_LINKS",
    guild_only,
    aliases("blacklist", "blkl"),
    identifying_name = "jUtils.moderation.ban"
)]
pub async fn ban(
    ctx: Context<'_>,
    #[description="The user to ban."]
    user: Member,
    #[description="The ban reason, if any."]
    reason: Option<String>
) -> Result<(), poise::serenity_prelude::Error> {
    _ = ctx.defer().await;

    let cu = ctx.http().get_current_user().await.expect("Expected a current user.");
    let rea: String = match reason {
        Some(str) => str,
        None => "No reason provided".to_owned()
    };
    let guild = ctx.http().get_guild(ctx.guild_id().expect("Expected GuildId")).await.expect("Expected Guild");
    let embed: CreateEmbed =
    if &user.user == ctx.author() {
        let em = CreateEmbed::error()
            .title("Failed to ban user")
            .description("You cannot ban yourself.");
        em
    } else if compare_roles(ctx.serenity_context(), guild, user.user.id).await {
        let em = CreateEmbed::error()
            .title("Failed to ban user")
            .description(format!("{} can't ban users that has a higher role!", cu.name));
        em
    } else if user.user.bot {
        let em = CreateEmbed::error()
            .title("Failed to ban user")
            .description("User is a bot.");
        em
    } else {
        match user.ban_with_reason(ctx.http(), 4, &format!("{}: {}", ctx.author().name, rea)).await {
            Ok(_) => {
                let em = CreateEmbed::success()
                    .title(format!("{} has been banned", user.display_name()))
                    .description(rea)
                    .field("Banned by", ctx.author().mention().to_string(), true);
                em
            }
            Err(e) => {
                let em = CreateEmbed::error()
                    .title("Failed to ban user")
                    .description(e.to_string());
                em
            }
        }
    };

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok(())
}
