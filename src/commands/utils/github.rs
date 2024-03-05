use chrono::DateTime;
use poise::serenity_prelude::Error;
use crate::types::Context;
use crate::EmbedHelper;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::*;
use poise::serenity_prelude::CreateAllowedMentions as am;
use reqwest::*;

#[poise::command(
    prefix_command,
    slash_command,
    category = "Utilities",
    track_edits,
    aliases("gh"),
    rename = "github"
)]
/// Get information about a GitHub repository or user.
pub async fn github(
    ctx: Context<'_>,
    #[description = "The GitHub repository or user you want to fetch"]
    name: String,
) -> serenity::Result<(), Error> {
    _ = ctx.defer().await;

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; jUtilsBot/1.0; +https://github.com/jbcarreon123/jUtils)")
        .build()?;

    let response: Response;
    if name.contains("/") {
        response = match client.get(&format!("https://api.github.com/repos/{}", name)).send().await {
            Ok(response) => response,
            Err(_) => {
                // If not a repository, assume it's a user
                match client.get(&format!("https://api.github.com/users/{}", name)).send().await {
                    Ok(response) => response,
                    Err(err) => {
                        return Err(err.into());
                    }
                }
            }
        };
    } else {
        response = match client.get(&format!("https://api.github.com/users/{}", name)).send().await {
            Ok(response) => response,
            Err(err) => {
                return Err(err.into());
            }
        }
    }

    if response.status().is_success() {
        let info: serde_json::Value = match response.json().await {
            Ok(info) => info,
            Err(err) => {
                let _ = ctx.reply("Failed to parse information.").await;
                return Err(err.into());
            }
        };

        let mut components = Vec::<CreateActionRow>::new();
        let mut btns = Vec::<CreateButton>::new();

        let embed = if info.get("full_name").is_some() {
            // Repository information
            CreateEmbed::primary()
                .title(format!("GitHub Repository: {}", name))
                .thumbnail(info["owner"]["avatar_url"].as_str().unwrap_or(""))
                .description(info["description"].as_str().unwrap_or("No description available"))
                .field("Language", info["language"].as_str().unwrap_or("Unknown"), true)
                .field("Stars", info["stargazers_count"].as_u64().unwrap_or(0).to_string(), true)
                .field("Forks", info["forks_count"].as_u64().unwrap_or(0).to_string(), true)
        } else {
            // User information
            CreateEmbed::primary()
                .title(format!("GitHub User: {}", name))
                .thumbnail(info["avatar_url"].as_str().unwrap_or(""))
                .description(info["bio"].as_str().unwrap_or("No bio available"))
                .field("Followers", info["followers"].as_u64().unwrap_or(0).to_string(), true)
                .field("Following", info["following"].as_u64().unwrap_or(0).to_string(), true)
                .field("Public Repos", info["public_repos"].as_u64().unwrap_or(0).to_string(), true)
        };

        if let Some(repo_url) = info["html_url"].as_str() {
            btns.push(CreateButton::new_link(repo_url).label(if info.get("full_name").is_some() { "Repository URL" } else { "User URL" }))
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
    } else {
        let _ = ctx.reply(format!("Failed to fetch information. Status code: {}", response.status())).await;
    }

    Ok(())
}