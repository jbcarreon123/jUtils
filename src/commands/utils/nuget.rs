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
    aliases("cspkg", "dotnetpkg"),
    rename = "nuget"
)]
/// Get information about a NuGet package.
pub async fn nuget(
    ctx: Context<'_>,
    #[description = "The NuGet package you want to fetch"]
    package_name: String,
) -> serenity::Result<(), Error> {
    _ = ctx.defer().await;

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; jUtilsBot/1.0; +https://github.com/jbcarreon123/jUtils)")
        .build()?;

    let response = match client.get(&format!("https://api.nuget.org/v3/registration3/{}/index.json", package_name)).send().await {
        Ok(response) => response,
        Err(err) => {
            return Err(err.into());
        }
    };

    if response.status().is_success() {
        let package_info: serde_json::Value = match response.json().await {
            Ok(package_info) => package_info,
            Err(err) => {
                let _ = ctx.reply("Failed to parse package information.").await;
                return Err(err.into());
            }
        };

        let mut components = Vec::<CreateActionRow>::new();
        let mut btns = Vec::<CreateButton>::new();

        let latest_version = package_info["items"][0]["items"][0]["catalogEntry"]["version"].as_str().unwrap_or("Unknown");

        let embed = CreateEmbed::primary()
            .title(format!("Package {}", package_name))
            .description(package_info["items"][0]["items"][0]["catalogEntry"]["description"].as_str().unwrap_or("No description available"))
            .field("Latest Version", latest_version, true);

        if let Some(package_url) = package_info["items"][0]["items"][0]["catalogEntry"]["projectUrl"].as_str() {
            btns.push(CreateButton::new_link(package_url).label("Project URL"))
        }
        if let Some(repository) = package_info["items"][0]["items"][0]["catalogEntry"]["repository"].as_object() {
            if let Some(repo_url) = repository["url"].as_str() {
                btns.push(CreateButton::new_link(repo_url).label("Repository"))
            }
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
        let _ = ctx.reply(format!("Failed to fetch package information. Status code: {}", response.status())).await;
    }

    Ok(())
}
