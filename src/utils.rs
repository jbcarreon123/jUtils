use poise::serenity_prelude as serenity;
use serenity::builder::CreateEmbed;
use serenity::EmbedField;
use poise::serenity_prelude::Http;
use serenity::CurrentUser;
use poise::serenity_prelude::Mentionable;
use tokio::time::error::Elapsed;
use crate::config;
use poise::serenity_prelude::json::Value;
use poise::Command;

pub async fn get_all_commands_as_embedfields<U, E>(
    ctx: poise::Context<'_, U, E>
) -> Result<Vec<Vec<(String, String, bool)>>, serenity::Error> {
    let config = config::load_config().expect("Expected the config to be found.");
    let mut commands = Vec::<&Command<U, E>>::new();
    for cmd in &ctx.framework().options().commands {
        commands
            .push(cmd);
    }

    let mut menu = Vec::<(String, String, bool)>::new();
    for command in commands {
        let desc: String = match &command.description {
            Some(string) => string.clone(),
            None => "No description".to_owned()
        };
        let pref = if ctx.prefix().starts_with("<") {
            config.discordbot.prefix.clone()
        } else {
            ctx.prefix().to_owned()
        };
        let space = if ctx.prefix().starts_with("<") {
            " "
        } else {
            ""
        };
        menu.push((
            format!("{}{}{}", pref, space, command.name),
            desc,
            true
        ))
    }

    Ok(chunk(menu, 25))
}

pub fn chunk<T: Clone>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    vec.into_iter()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>()
}
