pub mod commands;
pub mod types;

use std::env;
use types::*;
use commands::ping::*;
use poise::serenity_prelude as serenity;
use poise::CreateReply;
use poise::serenity_prelude::Embed;
use serenity::prelude::*;
use poise::structs;
use poise::serenity_prelude::ShardStageUpdateEvent;
use tracing::log::{ debug, info, warn, error, Level };

#[tokio::main]
async fn main() {
    let token = "MTAyNzIwMTc0NTY4NTg1NjI1Ng.Gz8-0J.kI8_fEvvnfWs2vgZ4fkJYWtcdlOekKsPLg0Wys";
    let intents = serenity::GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("b.".into()),
                mention_as_prefix: true,
                ..Default::default()
            },
            on_error: |error| {
                Box::pin(async move {
					if let poise::FrameworkError::ArgumentParse { error, ctx, .. } = error {
						let response = if error.is::<poise::CodeBlockError>() {
							"Missing code block."
								.to_owned()
						} else {
							format!("`{}`: {}", ctx.command().name, error.to_string())
						};

						if let Err(e) = ctx.say(response).await {
							warn!("{}", e)
						}
					} else if let poise::FrameworkError::Command { ctx, error, .. } = error {
						if let Err(e) = ctx.say(error.to_string()).await {
							warn!("{}", e)
						}
					}
				})
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}