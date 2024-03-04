use poise::serenity_prelude::CreateAllowedMentions as am;
use poise::serenity_prelude::CreateAllowedMentions;
use poise::serenity_prelude::json::Value;
use core::array::IntoIter;
use poise::serenity_prelude::Colour;
use poise::serenity_prelude::{CreateEmbed, CreateActionRow, CreateButton};

use crate::config;

pub struct Data {} type Error = poise::serenity_prelude::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub trait EmbedHelper {
    fn primary() -> CreateEmbed;
    fn secondary() -> CreateEmbed;
    fn success() -> CreateEmbed;
    fn error() -> CreateEmbed;
    fn invis() -> CreateEmbed;
}

impl EmbedHelper for CreateEmbed {
    fn primary() -> CreateEmbed {
        let config = config::load_config().expect("Expected the config to be found.");
        let embed = CreateEmbed::default()
            .color(Colour::from_rgb(config.colors.primary[0], config.colors.primary[1], config.colors.primary[2]));
        embed
    }
    fn secondary() -> CreateEmbed {
        let config = config::load_config().expect("Expected the config to be found.");
        let embed = CreateEmbed::default()
            .color(Colour::from_rgb(config.colors.secondary[0], config.colors.secondary[1], config.colors.secondary[2]));
        embed
    }
    fn success() -> CreateEmbed {
        let config = config::load_config().expect("Expected the config to be found.");
        let embed = CreateEmbed::default()
            .color(Colour::from_rgb(config.colors.success[0], config.colors.success[1], config.colors.success[2]));
        embed.to_owned()
    }
    fn error() -> CreateEmbed {
        let config = config::load_config().expect("Expected the config to be found.");
        let embed = CreateEmbed::default()
            .color(Colour::from_rgb(config.colors.error[0], config.colors.error[1], config.colors.error[2]));
        embed
    }
    fn invis() -> CreateEmbed {
        let config = config::load_config().expect("Expected the config to be found.");
        let embed = CreateEmbed::default()
            .color(Colour::from_rgb(config.colors.invis[0], config.colors.invis[1], config.colors.invis[2]));
        embed
    }
}