use poise::serenity_prelude::Error;
use serenity::all::CreateAllowedMentions as am;
use crate::{commands::user, database::{get_leveling, leveling::Leveling, save_leveling, UserLevel}, types::Context};
use std::marker::Send;

#[poise::command(
    prefix_command,
    hide_in_help,
    identifying_name = "jUtils.internal.init_leveling_test",
    owners_only,
    guild_only
)]
pub async fn init_leveling_test(
    ctx: Context<'_>
) -> Result<(), Error> {
    let mut user_levels = get_leveling(ctx.guild_id().unwrap().to_string()).await.unwrap();
    for user_level in &mut user_levels.levels {
        user_level.compute_level();   
    }
    save_leveling(user_levels).await.unwrap();
    ctx.send(poise::CreateReply::default()
        .content("done")
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}