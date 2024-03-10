use poise::serenity_prelude::Error;
use serenity::all::{ButtonStyle, CreateActionRow, CreateButton};
use crate::types::Context;
use poise::serenity_prelude::CreateAllowedMentions as am;

#[poise::command(
    prefix_command,
    hide_in_help,
    owners_only,
    identifying_name = "jUtils.internal.test"
)]
pub async fn test(
    ctx: Context<'_>
) -> Result<(), Error> {
    let mut btns = Vec::<CreateButton>::new();
    btns.push(CreateButton::new("test_btn_1")
        .label("Test Button 1")
        .style(ButtonStyle::Primary));
    btns.push(CreateButton::new("test_btn_2")
        .label("Test Button 2")
        .style(ButtonStyle::Secondary));
    btns.push(CreateButton::new("test_btn_3")
        .label("Test Button 3")
        .style(ButtonStyle::Success));
    btns.push(CreateButton::new("test_btn_4")
        .label("Test Button 4")
        .style(ButtonStyle::Danger));
    let ar = CreateActionRow::Buttons(btns);

    ctx.send(poise::CreateReply::default()
        .components([ ar ].to_vec())
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;
    Ok::<(), Error>(())
}