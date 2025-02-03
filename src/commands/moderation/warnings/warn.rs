use duration_string::DurationString;
use poise::serenity_prelude::Error as PoiseError;
use serenity::all::Member;
use serenity::all::Mentionable;
use crate::database::*;
use crate::types::Context;
use crate::utils::utils::*;
use crate::utils::time_utils::*;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;

/// Warn a user
#[poise::command(
    slash_command,
    prefix_command,
    category="Moderation",
    required_permissions = "MODERATE_MEMBERS",
    default_member_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "MODERATE_MEMBERS | SEND_MESSAGES | EMBED_LINKS",
    guild_only,
    aliases("strike"),
    identifying_name = "jUtils.moderation.warns.warn",
    check = "is_guild_configured",
)]
pub async fn warn(
    ctx: Context<'_>,
    #[description="The user to warn."]
    user: Member,
    #[description="The warn duration."]
    duration: String,
    #[description="The warn reason, if any."]
    reason: Option<String>
) -> Result<(), PoiseError> 
{
    _ = ctx.defer().await;
    let cu = ctx.http().get_current_user().await.expect("Expected a current user.");
    let rea: String = match reason {
        Some(str) => str,
        None => "No reason provided".to_owned()
    };
    let dur: std::time::Duration = DurationString::from_string(String::from(duration)).unwrap().into();
    let dt = duration_to_datetime(dur);
    let guild = ctx.http().get_guild(ctx.guild_id().expect("Expected GuildId")).await.expect("Expected Guild");
    let embed: CreateEmbed =
    if &user.user == ctx.author() {
        let em = CreateEmbed::error()
            .title("Failed to warn user")
            .description("You cannot warn yourself.");
        em
    } else if compare_roles(ctx.serenity_context(), guild, user.user.id).await {
        let em = CreateEmbed::error()
            .title("Failed to warn user")
            .description(format!("{} can't warn users that has a higher role!", cu.name));
        em
    } else if user.user.id == cu.id {
        let em = CreateEmbed::error()
            .title("Failed to warn user")
            .description("No.");
        em
    } else if user.user.bot {
        let em = CreateEmbed::error()
            .title("Failed to warn user")
            .description("User is a bot.");
        em
    } else {
        let em = match create_warn(ctx.author().id.to_string(), user.user.id.to_string(), rea.clone(), dt).await {
            Ok(id) => {
                CreateEmbed::success()
                    .title(format!("{} has been warned", user.user.name))
                    .description(rea)
                    .field("Warn ID", id, true)
                    .field("Duration", format_duration(dur), true)
                    .field("Warned by", ctx.author().mention().to_string(), true)
            }
            Err(err) => {
                CreateEmbed::error()
                    .title("Failed to warn user")
                    .description(err.to_string())
            }
        };
        em
    };

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), PoiseError>(())
}

