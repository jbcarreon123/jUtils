use std::path::Path;

use image::imageops::FilterType;
use image::DynamicImage;
use itertools::Itertools;
use serenity::futures::future::OrElse;
use serenity::model::colour;
use tokio::fs::File;
use poise::serenity_prelude::Error;
use serenity::all::{Colour, CreateAttachment, Member};
use crate::commands::leveling;
use crate::database::{get_user_leveling, get_user_rank};
use crate::types::Context;
use crate::utils::leveling::animation_utils::save_animation_to_file;
use crate::utils::leveling::generate_image::{generate_gif, generate_image, Background};
use crate::CONFIG;
use poise::serenity_prelude::CreateAllowedMentions as am;

#[poise::command(
    prefix_command,
    slash_command,
    hide_in_help,
    identifying_name = "jUtils.leveling.rank",
    track_edits,
    guild_only,
    aliases("rankcard", "xp", "level", "xpcard", "levelcard")
)]
pub async fn rank(
    ctx: Context<'_>,
    user: Option<Member>
) -> Result<(), Error> {
    _ = ctx.defer().await;
    let user = user.unwrap_or(ctx.author_member().await.unwrap().into_owned());

    let banner = match user.user.banner_url() {
        Some(i) => {
            let url = i.to_string();
            Background::Url(Box::leak(Box::new(url)))
        },
        None => Background::Color(user.user.accent_colour.unwrap_or(Colour::BLURPLE).0.to_rgba()),
    };
    let level = get_user_leveling(ctx.guild_id().unwrap().to_string(), user.user.id.to_string()).await.unwrap();
    let rank = get_user_rank(ctx.guild_id().unwrap().to_string(), user.user.id.to_string()).await.unwrap();
    let warning: &mut Option<String> = &mut None;
    
    let mut if_gif: bool = if CONFIG.lvlcard.allow_animated && CONFIG.lvlcard.allowed_users_animated.contains(&user.user.id.to_string()) {
        match banner {
            Background::Url(url) => url.contains(".gif"),
            _ => false,
        }
    } else {
        false
    };

    let file_exists = if Path::new(&format!(".leveling_temp/{}.{}", user.user.id.to_string(), if if_gif {"gif"} else {"png"})).exists() {
        true
    } else if Path::new(&format!(".leveling_temp/{}.{}", user.user.id.to_string(), "png")).exists() {
        true
    } else {
        false
    };
    
    let img_path = if file_exists {
        format!(".leveling_temp/{}.{}", user.user.id.to_string(), if if_gif {"gif"} else {"png"})
    } else {
        if !if_gif {
            let img = generate_image(&user.user.tag(), user.display_name(), level.level.try_into().unwrap(), level.xp as u32, level.compute_xp_required().try_into().unwrap(), rank as u32, ctx.guild().unwrap().name.as_str(), [224, 199, 133, 255], user.avatar_url().unwrap_or(user.user.avatar_url().unwrap_or(user.user.default_avatar_url())).as_str(), banner.clone(), warning);
            let img_path = ".leveling_temp/".to_string() + &user.user.id.to_string() + ".png";
            img.save(&img_path).unwrap();  
            img_path  
        } else {
            let tag = &user.user.tag();
            let guild = ctx.guild().unwrap();
            let guild_name = guild.name.as_str();
            let avatar_url = user.avatar_url().unwrap_or_else(|| user.user.avatar_url().unwrap_or_else(|| user.user.default_avatar_url()));
            let avatar_url = avatar_url.as_str();
            let img = generate_gif(tag, user.display_name(), level.level.try_into().unwrap(), level.xp as u32, level.compute_xp_required().try_into().unwrap(), rank as u32, guild_name, [224, 199, 133, 255], avatar_url, banner.clone(), warning);
            let mut img_path = ".leveling_temp/".to_string() + &user.user.id.to_string() + ".gif";
            if let Err(_) = save_animation_to_file(img.unwrap(), &img_path) {
                let banner_temp = banner;
                if_gif = false;
                let img = generate_image(&user.user.tag(), user.display_name(), level.level.try_into().unwrap(), level.xp as u32, level.compute_xp_required().try_into().unwrap(), rank as u32, ctx.guild().unwrap().name.as_str(), [224, 199, 133, 255], user.avatar_url().unwrap_or(user.user.avatar_url().unwrap_or(user.user.default_avatar_url())).as_str(), banner_temp, warning);
                img_path = ".leveling_temp/".to_string() + &user.user.id.to_string() + ".png";
                img.save(&img_path).unwrap();
            }
            img_path
        }
    };

    let warning = match warning {
        Some(w) => w.clone(),
        None => "".to_owned()
    };

    ctx.send(poise::CreateReply::default()
        .content(warning)
        .attachment(
            CreateAttachment::file(&File::open(img_path).await?, "output".to_string() + if !if_gif {".png"} else {".gif"}).await?
        )
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;

    Ok::<(), Error>(())
}

trait ToRgba {
    fn to_rgba(self) -> [u8; 4];
}

impl ToRgba for u32 {
    fn to_rgba(self) -> [u8; 4] {
        [
            ((self >> 16) & 0xFF) as u8, // Red
            ((self >> 8) & 0xFF) as u8,  // Green
            (self & 0xFF) as u8,         // Blue
            255,                         // Alpha
        ]
    }
}