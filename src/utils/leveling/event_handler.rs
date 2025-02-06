use std::arch::x86_64;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use itertools::Itertools;
use poise::async_trait;
use poise::serenity_prelude as serenity;
use ::serenity::all::ChannelId;
use ::serenity::all::CreateAllowedMentions as am;
use ::serenity::all::CreateEmbed;
use ::serenity::all::CreateEmbedAuthor;
use ::serenity::all::CreateMessage;
use ::serenity::all::Guild;
use ::serenity::all::GuildChannel;
use ::serenity::all::GuildId;
use ::serenity::all::Message;
use ::serenity::all::MessageId;
use ::serenity::all::MessageReference;
use serenity::prelude::*;
use poise::serenity_prelude::ActivityData;
use poise::serenity_prelude::Ready;
use rand::prelude::SliceRandom;
use crate::commands::user::user;
use crate::database::create_new_config;
use crate::database::get_guild_config;
use crate::database::get_leveling;
use crate::database::save_leveling;
use crate::database::UserLevel;
use crate::types::EmbedHelper;
use crate::CONFIG;

use super::anti_spam::ANTI_SPAM;

pub struct LevelingHandler;

#[async_trait]
impl EventHandler for LevelingHandler {
    async fn message(&self, _ctx: poise::serenity_prelude::Context, msg: Message) {
        if msg.author.bot || msg.author.system {
            return;
        } else if msg.guild_id.is_none() {
            return;
        } else if msg.content.len() < 3 {
            return;
        }

        let guild_id = msg.guild_id.unwrap();
        let user_id = msg.author.id;
        let channel_id = msg.channel_id;
        let guild = msg.guild(&_ctx.cache).unwrap().clone();
        let guild_user = guild.member(&_ctx.http, user_id).await.unwrap();

        let config = if let Ok(guild) = get_guild_config(guild_id.clone().to_string()).await {
            guild
        } else {
            return
        };

        if ANTI_SPAM.lock().await.is_spamming(&user_id.to_string()) {
            return
        }
        if config.modules.leveling.blacklisted_channels.contains(&channel_id.to_string()) {
            return
        }
        if config.modules.leveling.blacklisted_roles.iter().any(|r| guild_user.roles.iter().any(|o| r == &o.to_string())) {
            let user_roles: Vec<String> = guild_user.roles.iter().map(|o| o.to_string()).collect();
            if config.modules.leveling.blacklisted_roles.iter().any(|r| user_roles.contains(r)){
                return
            }
        }

        let mut user_levels = get_leveling(guild_id.to_string()).await.unwrap();
        let user_level: &mut UserLevel = if let Some(user) = user_levels.levels.iter_mut().find(|u| u.user_id == user_id.to_string()) {
            user
        } else {
            user_levels.levels.push(UserLevel::new(user_id.to_string()));
            user_levels.levels.last_mut().unwrap()
        };

        let xp_message = config.modules.leveling.xp_per_message.randomize() as f64 * config.modules.leveling.multiplier;

        println!("user: {}, ex-xp: {}, ad-xp: {}", user_id, user_level.xp, xp_message);

        user_level.add_xp(xp_message as i64);
        let prev_level = user_level.clone().level;
        user_level.compute_level();
        let now_level = user_level.clone().level;

        if prev_level != now_level {
            /* _ = channel_id.send_message(_ctx.http, 
                CreateMessage::new()
                .content(format!("user {} assumed level up, dbg: {}, {}", user_id, prev_level, now_level))
                .reference_message(MessageReference::new(serenity::MessageReferenceKind::Default, msg.channel_id)
                    .guild_id(guild_id)
                    .message_id(msg.id)
                )
                .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
            ).await;
            */
        }

        save_leveling(user_levels).await.unwrap();
    }
}