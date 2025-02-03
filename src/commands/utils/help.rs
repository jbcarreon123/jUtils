use poise::serenity_prelude::Error;
use crate::types::Context;


use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::utils::embed_utils::*;
use crate::utils::utils::*;
use crate::utils::command_utils::get_command;

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
	_ = ctx.defer().await;

	if command.is_none() {
		let fields = get_all_commands_as_embedfields(ctx).await.expect("Expected output");
		paginate(ctx, fields).await?;
	} else {
		let cmd = match get_command(ctx, command.unwrap()).await {
			Ok(t) => t,
			Err(e) => return Err(e)
		};

		ctx.send(poise::CreateReply::default()
			.embed(cmd)
			.reply(true)
			.allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
		).await?;
	}

	Ok(())
}