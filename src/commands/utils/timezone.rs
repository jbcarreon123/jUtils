use chrono_tz::TZ_VARIANTS;
use poise::serenity_prelude::Error;
use crate::database::set_user_timezone;
use crate::types::Context;
use poise::serenity_prelude::CreateAllowedMentions as am;

#[poise::command(
    prefix_command,
    slash_command,
    category="Utilities",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    identifying_name = "jUtils.utils.timezone",
    subcommands("set"),
    subcommand_required
)]
pub async fn timezone(
    _ctx: Context<'_>,
) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    category="Utilities",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    identifying_name = "jUtils.utils.timezone.set",
)] 
pub async fn set(
    ctx: Context<'_>,
    #[description = "The tz timezone name you want to set"]
    #[autocomplete = "timezone_autocomplete"]
    timezone: String,
) -> Result<(), Error> {
    set_user_timezone(ctx.author().id.to_string(), timezone).await;
    ctx.send(poise::CreateReply::default()
        .content("Timezone set!")
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok(())
}

async fn timezone_autocomplete<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
    let timezones = TZ_VARIANTS.iter().map(|tz| {
        let now = chrono::Utc::now().with_timezone(tz);
        format!("{} - {}", tz, now.format("%H:%M (%I:%M %p)"))
    }).collect::<Vec<String>>();
    timezones
        .into_iter()
        .filter(move |tz| tz.starts_with(partial))
        .map(String::from)
}