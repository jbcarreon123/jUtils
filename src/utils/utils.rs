use itertools::Itertools;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateAllowedMentions as am;
use poise::serenity_prelude::PartialGuild;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;
use ::serenity::all::CacheHttp;
use ::serenity::all::CreateActionRow;
use ::serenity::all::CreateButton;
use ::serenity::all::CreateEmbed;
use ::serenity::all::EditChannel;
use ::serenity::all::EmojiId;
use ::serenity::all::GuildChannel;
use ::serenity::all::Http;
use ::serenity::all::Member;
use ::serenity::all::Permissions;
use ::serenity::all::ReactionType;
use std::str::FromStr;
use crate::EmbedHelper;
use crate::CONFIG;
use poise::serenity_prelude::Context;
use poise::serenity_prelude::UserId;
use chrono::DateTime;

// Removed format_duration, duration_to_rfc3339, and duration_to_datetime functions

pub async fn compare_roles(ctx: &Context, guild: PartialGuild, user_id: UserId) -> bool {
    let cu = ctx.http.get_current_user().await.expect("Expected a current user.");
    let bot_member = match guild.member(ctx.http.clone(), cu.id).await {
        Ok(member) => member,
        Err(_) => return false
    };
    let user_member = match guild.member(ctx.http.clone(), user_id).await {
        Ok(member) => member,
        Err(_) => return false,
    };

    let bhighest_role = bot_member.highest_role_info(ctx.cache.clone()).expect("Expected roles");
    let uhighest_role = user_member.highest_role_info(ctx.cache.clone()).expect("Expected roles");

    if bhighest_role.1 < uhighest_role.1 {
        return true
    }
    false
}

pub async fn fetch_package_info(base_url: &str, package_name: &str, suffix: &str) -> Result<serde_json::Value, serenity::Error> {
    let client = reqwest::Client::builder()
        .user_agent("Your User-Agent Here")
        .build()?;

    let response = client.get(&format!("{}{}{}", base_url, package_name, suffix))
        .send()
        .await?;

    if response.status().is_success() {
        let data = response.json::<serde_json::Value>().await?;
        Ok(data)
    } else {
        let t = match response.text().await {
			Ok(s) => s,
			Err(_) => "".to_owned()
		};
        println!("{}", t.clone());
        Err(serenity::Error::Other(Box::leak(t.into_boxed_str())))
    }
}

pub async fn edit_channel_name(mut chan: GuildChannel, http: &Http, name: String) {
    chan.edit(http, EditChannel::new().name(name)).await;
}

pub async fn send_package_info<'a, U, E>(ctx: poise::Context<'a, U, E>, package_info: serde_json::Value, package_name: &'a str) -> Result<(), serenity::Error> {
    let mut components = Vec::<CreateActionRow>::new();
    let mut btns = Vec::<CreateButton>::new();

    let created_at = DateTime::parse_from_rfc3339(package_info["created_at"].as_str().expect("Expected created at")).unwrap();
    let created_at_us = created_at.timestamp();

    let lv_published_at = DateTime::parse_from_rfc3339(package_info["versions"][0]["created_at"].as_str().expect("Expected created at")).unwrap();
    let lv_published_at_us = lv_published_at.timestamp();

    let embed = CreateEmbed::primary()
        .title(format!("Package {}", package_name))
        .description(package_info["description"].as_str().unwrap_or("No description available"))
        .field("Published at", format!("<t:{}>", created_at_us), true)
        .field("Downloads", package_info["downloads"].as_u64().unwrap_or(0).to_string(), true)
        .field("Latest Version", format!("{:?}\nPublished at <t:{:?}>\n{:?} downloads",
            package_info["versions"][0]["num"].as_str(),
            lv_published_at_us,
            package_info["versions"][0]["downloads"].clone().take().as_u64()), true);

    if let Some(documentation) = package_info["documentation"].as_str() {
        btns.push(CreateButton::new_link(documentation).label("Documentation"))
    }
    if let Some(homepage) = package_info["homepage"].as_str() {
        btns.push(CreateButton::new_link(homepage).label("Homepage"))
    }
    if let Some(repository) = package_info["repository"].as_str() {
        btns.push(CreateButton::new_link(repository).label("Repository"))
    }
    components.push(CreateActionRow::Buttons(btns.clone()));

    if !btns.is_empty() {
        ctx.send(poise::CreateReply::default()
            .embed(embed)
            .components(components)
            .reply(true)
            .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
        ).await?;
    } else {
        ctx.send(poise::CreateReply::default()
            .embed(embed)
            .reply(true)
            .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
        ).await?;
    }

    Ok(())
}

pub fn generate_id(num: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(num)
        .map(char::from)
        .collect()
}

pub async fn paginate<U, E>(
    ctx: poise::Context<'_, U, E>,
    pages: Vec<Vec<(String, String, bool)>>,
) -> Result<(), serenity::Error> {
    let ctx_id = ctx.id();
    let prefix = format!("jutils.help.{}", ctx_id);
    let prev_button_id = format!("{}.prev", prefix.clone());
    let next_button_id = format!("{}.next", prefix.clone());
    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&prev_button_id).emoji('◀'),
            serenity::CreateButton::new(&next_button_id).emoji('▶'),
        ]);

        let cu = ctx.http().get_current_user().await.expect("Expected a current user.");
        let embed = CreateEmbed::primary()
			.title(format!("Help for {}", cu.name))
			.fields(pages[0].clone());

        poise::CreateReply::default()
			.embed(embed)
            .components([ components ].to_vec())
			.reply(true)
			.allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    };
    ctx.send(reply).await?;
    let mut current_page: usize = 0;
    while let Some(press) = {
        let prefix_clone = prefix.clone();
        serenity::collector::ComponentInteractionCollector::new(ctx)
            .filter(move |press| press.data.custom_id.starts_with(prefix_clone.as_str()))
            .timeout(std::time::Duration::from_secs(3600 * 24))
            .await
    } {
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            continue;
        }
        let cu = ctx.http().get_current_user().await.expect("Expected a current user.");
        let embed = CreateEmbed::primary()
			.title(format!("Help for {}", cu.name))
			.fields(pages[current_page].clone());
        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(embed),
                ),
            )
            .await?;
    }

    Ok(())
}

pub async fn paginate_leaderboard<U, E>(
    ctx: poise::Context<'_, U, E>,
    pages: Vec<Vec<(String, String, bool)>>,
) -> Result<(), serenity::Error> {
    let ctx_id = ctx.id();
    let prefix = format!("jutils.leaderboard.{}", ctx_id);
    let prev_button_id = format!("{}.prev", prefix.clone());
    let next_button_id = format!("{}.next", prefix.clone());
    
    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&prev_button_id).emoji('◀'),
            serenity::CreateButton::new(&next_button_id).emoji('▶'),
        ]);

        let cu = ctx.guild().expect("Expected guild").clone();
        let embed = CreateEmbed::primary()
            .title(format!("Leaderboard for {}", cu.name))
            .fields(pages[0].clone());

        poise::CreateReply::default()
            .embed(embed)
            .components([components].to_vec())
            .reply(true)
            .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    };
    ctx.send(reply).await?;
    let mut current_page = 0;
    while let Some(press) = {
        let prefix_clone = prefix.clone();
        serenity::collector::ComponentInteractionCollector::new(ctx)
            .filter(move |press| press.data.custom_id.starts_with(prefix_clone.as_str()))
            .timeout(std::time::Duration::from_secs(3600 * 24))
            .await
    } {
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            continue;
        }
        let cu = ctx.guild().expect("Expected guild").clone();
        let embed = CreateEmbed::primary()
            .title(format!("Leaderboard for {}", cu.name))
            .fields(pages[current_page].clone());
        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(embed),
                ),
            )
            .await?;
    }

    Ok(())
}

pub trait BoolHelper {
    fn to_reaction(self) -> ReactionType;
    fn to_reaction_str(self) -> String;
    fn to_string(self) -> &'static str;
}

impl BoolHelper for bool {
    fn to_reaction(self) -> ReactionType {
        let parts: Vec<&str>;
        if self {
            let n = &CONFIG.emoji.check_box;
            let cleaned_input = n.trim_start_matches("<:").trim_end_matches(">");
            parts = cleaned_input.split(':').collect();
        } else {
            let n = &CONFIG.emoji.cross_box;
            let cleaned_input = n.trim_start_matches("<:").trim_end_matches(">");
            parts = cleaned_input.split(':').collect();
        }

        ReactionType::Custom {
            animated: false,
            id: EmojiId::from_str(parts[1]).unwrap(),
            name: Some(parts[0].to_owned())
        }
    }

    fn to_reaction_str(self) -> String {
        if self {
            CONFIG.emoji.check_box.clone()
        } else {
            CONFIG.emoji.cross_box.clone()
        }
    }

    fn to_string(self) -> &'static str {
        if self {
            "True"
        } else {
            "False"
        }
    }
}

pub async fn can_moderate(
    guild: &PartialGuild,
    punisher_member: &Member,
    target_member: &Member,
) -> Result<String, serenity::Error> {
    let mut reason: String = String::new();

    if punisher_member.user.id == target_member.user.id {
        reason = "You cannot moderate yourself.".to_string();
    }

    if target_member.permissions.iter().contains(&Permissions::ADMINISTRATOR) {
        reason = "The target is an administrator.".to_string();
    }

    if guild.owner_id == target_member.user.id {
        reason = "The target is the owner of the guild.".to_string();
    }

    Ok(reason)
}
