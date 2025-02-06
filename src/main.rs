pub mod commands;
pub mod types;
pub mod config;
pub mod utils;
pub mod send;
pub mod database;
mod error;
mod events;

use commands::*;
use config::Config;
use error::error_event;
use events::Handler;
use types::*;
use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use once_cell::sync::Lazy;
use utils::leveling::event_handler::LevelingHandler;
pub static CONFIG: Lazy<Config> = Lazy::new(|| config::load_config().expect("Expected the config to be found."));

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
                timezone::timezone(),
                nuget::nuget(),
                pypi::pypi(),
                github::github(),
                send_to_bots_behalf::stbb(),
                warn::warn(),
                kick::kick(),
                ban::ban(),
                list_warns::warns(),
                invite::invite(),
                guild::guild(),
                user::user(),
                massedit_channel::massedit_channel(),

                permissions::permissions(),
                guild_config::config::config(),
                guild_config::config_to::config_to(),
                guild_config::config_to::json(),
                guild_config::config_to::toml(),

                ee::roc(),
                ee::rock(),
                ee::gowthr(),
                ee::utils(),
                ee::b(),

                rank::rank(),
                leaderboard::leaderboard(),

                init_leveling_test::init_leveling_test(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(CONFIG.discordbot.prefix.clone().into()),
                mention_as_prefix: true,
                ..Default::default()
            },
            on_error: |error| {
                Box::pin(async move {
					error_event(error).await
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
        .event_handler(LevelingHandler)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}