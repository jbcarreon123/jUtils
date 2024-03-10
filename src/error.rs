use tracing::log::warn;
use poise::serenity_prelude::CreateAllowedMentions as am;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use poise::FrameworkError;
use poise::serenity_prelude::Error;

use crate::Data;
use crate::EmbedHelper;

pub async fn error_event(error: FrameworkError<'_, Data, Error>) {
    if let poise::FrameworkError::ArgumentParse { error, ctx, .. } = error {
        let embed = CreateEmbed::error()
            .title("Argument parse error")
            .description(error.to_string());

        if let Err(e) = ctx.send(poise::CreateReply::default()
            .embed(embed)
            .reply(true)
            .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
        ).await {
            warn!("{}", e)
        }
    } else if let poise::FrameworkError::Command { ctx, error, .. } = error {
        let embed = CreateEmbed::error()
            .title("An error occured while running the command!")
            .description(error.to_string());

        if let Err(e) = ctx.send(poise::CreateReply::default()
            .embed(embed)
            .reply(true)
            .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
        ).await {
            warn!("{}", e)
        }
    } else if let poise::FrameworkError::MissingUserPermissions { missing_permissions, ctx, .. } = error {
        let perms = missing_permissions.expect("Permissions expected");
        
        let permissions_names = perms.iter_names().map(|name| name.0).collect::<Vec<&str>>().join(", ");
        let embed = CreateEmbed::error()
            .title("Access denied")
            .description("You don't have enough permissions to use this command!")
            .field("Required Permissions", permissions_names, true);

        if let Err(e) = ctx.send(poise::CreateReply::default()
            .embed(embed)
            .reply(true)
            .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
        ).await {
            warn!("{}", e)
        }
    } else if let poise::FrameworkError::CommandPanic { ctx, .. } = error {
        let embed = CreateEmbed::error()
            .title("A panic has occured while executing this command!");

        if let Err(e) = ctx.send(poise::CreateReply::default()
            .embed(embed)
            .reply(true)
            .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
        ).await {
            warn!("{}", e)
        }
    } else if let poise::FrameworkError::UnknownCommand { ctx, msg, msg_content,  .. } = error {
        let cmd = msg_content.split(" ").next();

        if let Err(e) = msg.reply(ctx.http(), format!("`{}`: Unknown command", cmd.unwrap_or("{unknown}"))).await {
            warn!("{}", e)
        }
    }
}