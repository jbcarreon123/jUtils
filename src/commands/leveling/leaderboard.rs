use image::imageops::FilterType;
use image::DynamicImage;
use serenity::model::{colour, guild};
use tokio::fs::File;
use poise::serenity_prelude::Error;
use serenity::all::{Colour, CreateAttachment, Member};
use crate::commands::{leveling, user};
use crate::database::{get_leaderboard, get_user_leveling, get_user_rank};
use crate::types::Context;
use crate::utils::leveling::generate_image::{generate_image, Background};
use crate::utils::utils::paginate_leaderboard;
use poise::serenity_prelude::CreateAllowedMentions as am;

#[poise::command(
    prefix_command,
    slash_command,
    hide_in_help,
    identifying_name = "jUtils.leveling.leaderboard",
    track_edits,
    guild_only
)]
pub async fn leaderboard(
    ctx: Context<'_>
) -> Result<(), Error> {
    _ = ctx.defer().await;

    let fields = get_leaderboard_as_embedfields(ctx.guild_id().unwrap().to_string(), ctx.author().id.clone().to_string()).await;

    paginate_leaderboard(ctx, fields).await?;

    Ok::<(), Error>(())
}

pub async fn get_leaderboard_as_embedfields(
    guild_id: String,
    user_id: String
) -> Vec<Vec<(String, String, bool)>> {
    let leaderboard = get_leaderboard(guild_id).await.unwrap();
    let mut fields = Vec::new();
    for (i, user) in leaderboard.iter().enumerate() {
        if i % 12 == 0 {
            fields.push(Vec::new());
        }
        let rank = (i + 1).to_string() + &format!(": Level {}, XP: {}", user.level, user.xp);
        let user_info = format!("<@{}>{}", user.user_id, if user.user_id == user_id { " (You)" } else { "" });
        fields.last_mut().unwrap().push((rank, user_info, true));
    }
    fields
}
