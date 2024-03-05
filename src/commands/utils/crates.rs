use chrono::DateTime;
use poise::serenity_prelude::Error;
use crate::types::Context;
use crate::EmbedHelper;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::*;
use poise::serenity_prelude::CreateAllowedMentions as am;

#[poise::command(
    prefix_command,
    slash_command,
    category = "Utilities",
    track_edits,
    rename = "crate",
    aliases("cargo")
)]
/// Get information about a Rust crate from crates.io.
pub async fn crates(
    ctx: Context<'_>,
    #[description = "The crate you want to fetch"]
    crate_name: String,
) -> serenity::Result<(), Error> {
    _ = ctx.defer().await;

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; jUtilsBot/1.0; +https://github.com/jbcarreon123/jUtils)")
        .build()?;

    let response = match client.get(&format!("https://crates.io/api/v1/crates/{}", crate_name)).send().await {
        Ok(response) => response,
        Err(err) => {
            return Err(err.into());
        }
    };

    // Check if the request was successful
    if response.status().is_success() {
        let crate_info: serde_json::Value = match response.json().await {
            Ok(crate_info) => crate_info,
            Err(err) => {
                let _ = ctx.reply("Failed to parse crate information.").await;
                return Err(err.into());
            }
        };

		let mut components = Vec::<CreateActionRow>::new();
		let mut btns = Vec::<CreateButton>::new();

		let created_at = DateTime::parse_from_rfc3339(crate_info["crate"]["created_at"].as_str().expect("Expected created at")).unwrap();
    	let created_at_us = created_at.timestamp();

		let lv_published_at = DateTime::parse_from_rfc3339(crate_info["versions"][0]["created_at"].as_str().expect("Expected created at")).unwrap();
    	let lv_published_at_us = lv_published_at.timestamp();

        let embed = CreateEmbed::primary()
            .title(format!("Crate {}", crate_name))
            .description(crate_info["crate"]["description"].as_str().unwrap_or("No description available"))
            .field("Published at", format!("<t:{}>", created_at_us), true)
            .field("Downloads", crate_info["crate"]["downloads"].as_u64().unwrap_or(0).to_string(), true)
            .field("Latest Version", format!("{:?}\nPublished at <t:{:?}>\n{:?} downloads",
				crate_info["versions"][0]["num"].as_str(),
				lv_published_at_us,
				crate_info["versions"][0]["downloads"].clone().take().as_u64()), true);

        if let Some(documentation) = crate_info["crate"]["documentation"].as_str() {
            btns.push(CreateButton::new_link(documentation).label("Documentation"))
        }
        if let Some(homepage) = crate_info["crate"]["homepage"].as_str() {
            btns.push(CreateButton::new_link(homepage).label("Homepage"))
        }
        if let Some(repository) = crate_info["crate"]["repository"].as_str() {
            btns.push(CreateButton::new_link(repository).label("Repository"))
        }
		components.push(CreateActionRow::Buttons(btns.clone()));

        if btns.len() > 0 {
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
    } else {
		let r = response;
		let t = match r.text().await {
			Ok(s) => s,
			Err(_) => "".to_owned()
		};
        let _ = ctx.reply(format!("Failed to fetch crate information. {}", t)).await;
    }

    Ok(())
}
