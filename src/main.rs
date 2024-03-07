pub mod commands;
pub mod types;
pub mod config;
pub mod utils;
pub mod send;
pub mod database;

use commands::*;

use config::Config;
use database::load_db;
use rand::seq::SliceRandom;
use std::thread;
use poise::serenity_prelude::CreateEmbed;
use std::time::Duration;
use poise::serenity_prelude::CreateAllowedMentions as am;
use poise::async_trait;
use types::*;
use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use poise::serenity_prelude::ActivityData;
use poise::serenity_prelude::Ready;
use tracing::log::warn;
use once_cell::sync::Lazy;

struct Handler;

pub static CONFIG: Lazy<Config> = Lazy::new(|| config::load_config().expect("Expected the config to be found."));

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: poise::serenity_prelude::Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let strs: &Vec<String> = &CONFIG.motd.motd_strings;
        let mut strings = strs.clone();

        let _handle = thread::spawn(move || {
            for _i in 0.. {
                if strings.is_empty() {
                    println!("The vector is empty!");
                    return;
                }
                let mut rng = rand::thread_rng();
                strings.shuffle(&mut rng);
                let helpstr = if CONFIG.motd.include_help_prefix {
                    format!("{}help | ", CONFIG.discordbot.prefix)
                } else {
                    String::new()
                };

                ctx.shard.set_activity(Some(ActivityData::custom(format!("{}{}", helpstr, strings.choose(&mut rng).unwrap()))));
                thread::sleep(Duration::from_secs(CONFIG.motd.motd_timeout));
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let token: &str = &CONFIG.discordbot.token;
    let intents = serenity::GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                ping::ping(),
                help::help(),
                about::about(),
                timeout::timeout(),
                crates::crates(),
                npm::npm(),
                nuget::nuget(),
                pypi::pypi(),
                github::github(),
                send_to_bots_behalf::stbb(),
                warn::warn(),
                kick::kick(),
                ban::ban(),
                list_warns::warns(),
                permissions::permissions(),

                ee::roc(),
                ee::gowthr()
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(CONFIG.discordbot.prefix.clone().into()),
                mention_as_prefix: true,
                ..Default::default()
            },
            on_error: |error| {
                Box::pin(async move {
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

                        if let Err(e) = msg.reply(ctx.http(), format!("There is no such command called **{}**!", cmd.unwrap_or("{unknown}"))).await {
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