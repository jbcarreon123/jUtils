use std::time::Duration;

use poise::serenity_prelude::Error;
use poise::CreateReply;
use serenity::all::ChannelId;
use serenity::all::CreateActionRow;
use serenity::all::CreateButton;
use serenity::all::CreateMessage;
use serenity::all::GuildId;
use tokio::time::sleep;
use crate::send::get_msg_data_from_json_str;
use crate::types::Context;
use poise::serenity_prelude::CreateEmbed;

#[poise::command(
    prefix_command,
    hide_in_help,
    owners_only,
    track_deletion,
    identifying_name = "jUtils.internal.stbb"
)]
pub async fn stbb(
    ctx: Context<'_>,
    json: String
) -> Result<(), Error> {
    let j = get_msg_data_from_json_str(&json).expect("Expected MessageData, but found Error");
    let mut e = CreateEmbed::default()
        .title(j.embed.title)
        .description(j.embed.description)
        .url(j.embed.url.unwrap_or("".to_owned()))
        .image(j.embed.image.unwrap_or("".to_owned()))
        .thumbnail(j.embed.thumbnail.unwrap_or("".to_owned()));

    for f in j.embed.fields {
        e = e.clone().field(f.name, f.value, f.inline)
    }

    let mut components = Vec::<CreateActionRow>::new();
    let mut btns = Vec::<CreateButton>::new();
    let mut c = 0;
    for b in j.link_components {
        c += 1;
        btns.push(CreateButton::new_link(b.url).label(b.name));
        if c >= 5 {
            components.push(CreateActionRow::Buttons(btns));
            btns = Vec::new();
        }
    }
    components.push(CreateActionRow::Buttons(btns));

    let g = ctx.http().get_guild(GuildId::new(j.guild_id)).await.expect("Expected guild");
    let ch = g.channels(ctx.http()).await.expect("Expected channels to be found");
    let c = ch.iter().find(|p| p.0 == &ChannelId::new(j.channel_id)).expect("Expected the channel to be found").1;

    _ = c.send_message(ctx.http(), CreateMessage::default()
        .add_embed(e)
        .components(components)).await;

    _ = ctx.send(CreateReply::default().content("Done!")).await;
    Ok::<(), Error>(())
}