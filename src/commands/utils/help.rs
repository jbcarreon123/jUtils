use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Error;
use poise::serenity_prelude::Mentionable;
use crate::types::Context;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::json::Value;
use crate::config;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::utils::*;

#[poise::command(
	prefix_command,
	slash_command,
	category = "Utilities",
	track_edits
)]
/// Get help from using the bot.
pub async fn help(
	ctx: Context<'_>,
	#[description = "Specific command to show help about"]
	#[autocomplete = "poise::builtins::autocomplete_command"]
	command: Option<String>,
) -> Result<(), Error> {
    let cu = ctx.http().get_current_user().await.expect("Expected a current user.");
	let fields = get_all_commands_as_embedfields(ctx).await.expect("Expected output");
	let embed = CreateEmbed::default()
		.title(format!("Help for {}", cu.name))
		.description("test")
		.fields(fields[0].clone());
    
	ctx.send(poise::CreateReply::default()
		.embed(embed)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;

	Ok(())
}