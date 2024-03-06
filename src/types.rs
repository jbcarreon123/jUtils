use poise::serenity_prelude::Colour;
use poise::serenity_prelude::CreateEmbed;

use crate::CONFIG;

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
        let embed = CreateEmbed::default()
            .color(Colour::from_rgb(CONFIG.colors.primary[0], CONFIG.colors.primary[1], CONFIG.colors.primary[2]));
        embed
    }
    fn secondary() -> CreateEmbed {
        let embed = CreateEmbed::default()
            .color(Colour::from_rgb(CONFIG.colors.secondary[0], CONFIG.colors.secondary[1], CONFIG.colors.secondary[2]));
        embed
    }
    fn success() -> CreateEmbed {
        let embed = CreateEmbed::default()
            .color(Colour::from_rgb(CONFIG.colors.success[0], CONFIG.colors.success[1], CONFIG.colors.success[2]));
        embed.to_owned()
    }
    fn error() -> CreateEmbed {
        let embed = CreateEmbed::default()
            .color(Colour::from_rgb(CONFIG.colors.error[0], CONFIG.colors.error[1], CONFIG.colors.error[2]));
        embed
    }
    fn invis() -> CreateEmbed {
        let embed = CreateEmbed::default()
            .color(Colour::from_rgb(CONFIG.colors.invis[0], CONFIG.colors.invis[1], CONFIG.colors.invis[2]));
        embed
    }
}