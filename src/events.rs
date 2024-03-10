use std::thread;
use std::time::Duration;

use poise::async_trait;
use poise::serenity_prelude as serenity;
use ::serenity::all::Guild;
use serenity::prelude::*;
use poise::serenity_prelude::ActivityData;
use poise::serenity_prelude::Ready;
use rand::prelude::SliceRandom;


use crate::database::create_new_config;

use crate::CONFIG;

pub struct Handler;

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

    async fn guild_create(&self, _ctx: poise::serenity_prelude::Context, guild: Guild, is_new: Option<bool>) {
        if is_new.unwrap() {
            _ = create_new_config(guild.id.to_string()).await;
        }
    }
}