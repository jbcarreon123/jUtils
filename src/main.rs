pub mod commands;
pub mod types;
pub mod config;
pub mod utils;

use rand::seq::SliceRandom;
use std::thread;
use std::time::Duration;
use poise::async_trait;
use types::*;
use commands::utils::*;
use commands::about::*;
use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use poise::serenity_prelude::ActivityData;
use poise::serenity_prelude::Ready;
use tracing::log::{ debug, info, warn, error, Level };

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: poise::serenity_prelude::Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let config = config::load_config().expect("Expected the config to be found.");
        let mut strings = config.motd.motd_strings;

        let handle = thread::spawn(move || {
            for i in 0.. {
                if strings.is_empty() {
                    println!("The vector is empty!");
                    return;
                }
                let mut rng = rand::thread_rng();
                strings.shuffle(&mut rng);
                let helpstr = if config.motd.include_help_prefix {
                    format!("{}help | ", config.discordbot.prefix)
                } else {
                    String::new()
                };

                ctx.shard.set_activity(Some(ActivityData::custom(format!("{}{}", helpstr, strings.choose(&mut rng).unwrap()))));
                thread::sleep(Duration::from_secs(config.motd.motd_timeout));
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let config = config::load_config().expect("Expected the config to be found.");
    let token = config.discordbot.token;
    let intents = serenity::GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping::ping(), help::help(), about()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.discordbot.prefix.into()),
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
							format!("`{}{}`: {}", ctx.prefix(), ctx.command().name, error.to_string())
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
        .event_handler(Handler)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}