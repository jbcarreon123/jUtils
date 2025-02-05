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
use ::serenity::all::MessageId;
use ::serenity::all::MessageReference;
use serenity::prelude::*;
use poise::serenity_prelude::ActivityData;
use poise::serenity_prelude::Ready;
use rand::prelude::SliceRandom;
use crate::database::create_new_config;
use crate::types::EmbedHelper;
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

    async fn message(&self, _ctx: poise::serenity_prelude::Context, msg: serenity::Message) {
        if msg.author.bot {
            return;
        } else if msg.guild_id.is_none() {
            return;
        }

        let msg_content = &msg.content;
        let discord_message_link_regex = regex::RegexBuilder::new(r"https://discord.com/channels/(\d+)/(\d+)/(\d+)")
            .multi_line(true)
            .case_insensitive(true)
            .build()
            .unwrap();
        for cap in discord_message_link_regex.captures_iter(msg_content) {
            let guild_id = GuildId::from(cap[1].parse::<u64>().unwrap());
            let channel_id = ChannelId::from(cap[2].parse::<u64>().unwrap());
            let message_id = MessageId::from(cap[3].parse::<u64>().unwrap());

            if msg.guild_id.unwrap() == guild_id {
                let guild = _ctx.http.get_guild(guild_id).await.unwrap();
                if let Some((_, channel)) = guild.channels(&_ctx.http).await.unwrap().iter().find(|(id, _)| id == &&channel_id) {
                    if let Ok(message) = channel.message(&_ctx.http, &message_id).await {
                        let mut embed = CreateEmbed::primary()
                            .author(CreateEmbedAuthor::new(&message.author.name)
                                        .icon_url(message.author.avatar_url().unwrap_or(message.author.default_avatar_url()))
                                    )
                            .url(message.link())
                            .title(format!("Message `{}` in {}", message_id, channel.name))
                            .description(message.content)
                            .field("Pinned?", message.pinned.to_string(), true)
                            .field("Sent by an app?", message.application_id.is_some().to_string(), true);

                        if let Some(reference) = message.message_reference {
                            embed = embed.field("Reference", format!("https://discord.com/channels/{}/{}/{}", reference.guild_id.unwrap().to_string(), reference.channel_id.to_string(), reference.message_id.unwrap().to_string()), false)
                        }

                        if !message.attachments.is_empty() {
                            embed = embed.field("Attachments", message.attachments.iter().map(|a| format!("[{}]({})", a.filename, a.url)).join(", "), false)
                        }

                        if let serenity::Channel::Guild(chnl) = msg.channel(&_ctx.http).await.unwrap() {
                            _ = chnl.send_message(&_ctx.http, 
                                CreateMessage::new()
                                    .add_embed(embed)
                                    .add_embeds(message.embeds.into_iter().map(|e| CreateEmbed::from(e)).collect())
                                    .reference_message(MessageReference::new(serenity::MessageReferenceKind::Default, msg.channel_id)
                                        .guild_id(guild_id)
                                        .message_id(msg.id)
                                    )
                                    .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
                            ).await;
                        }
                    }   
                }
            }
        }
    }
}