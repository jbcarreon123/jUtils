use shuttle_secrets::SecretStore;
use poise::serenity_prelude as serenity;

pub struct Data {} type Error = poise::serenity_prelude::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;