use std::str::FromStr;
use poise::serenity_prelude::Error;
use serenity::all::ComponentInteractionDataKind;
use serenity::all::ChannelType;
use serenity::all::CreateActionRow;
use serenity::all::CreateButton;
use serenity::all::CreateSelectMenu;
use serenity::all::CreateSelectMenuKind;
use serenity::all::CreateSelectMenuOption;
use serenity::all::EditMessage;
use serenity::all::EmojiId;
use serenity::all::ReactionType;
use serenity::futures::StreamExt;
use crate::database::get_guild_config;
use crate::database::create_new_config;
use crate::types::Context;
use crate::utils::utils::BoolHelper;
use crate::CONFIG;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateAllowedMentions as am;
use crate::types::EmbedHelper;

#[derive(PartialEq, Debug, poise::ChoiceParameter)]
pub enum Category {
    Moderation,
    ModerationNameBan,
    ModerationAutoModHook,
    ModerationReports,
    ServerQoL,
    Utilities,
    UtilitiesWiki,
    Leveling,
    Starboard,
    Counting,
    Tickets,
    Tagging,
    ServerStats,
    None
}

impl Category {
    fn to_friendly_name(&self) -> &'static str {
        match self {
            Category::Moderation => "Moderation",
            Category::ModerationNameBan => "Moderation > Name Ban",
            Category::ModerationAutoModHook => "Moderation > Discord AutoMod Hook",
            Category::ModerationReports => "Moderation > Reports",
            Category::ServerQoL => "Server Quality of Life",
            Category::Utilities => "Utilities",
            Category::UtilitiesWiki => "Utilities > Wiki",
            Category::Leveling => "Leveling",
            Category::Starboard => "Starboard",
            Category::Counting => "Counting",
            Category::Tickets => "Tickets",
            Category::Tagging => "Tagging",
            Category::ServerStats => "Server Statistics",
            Category::None => ""
        }
    }

    fn from_friendly_name(inp: Option<String>) -> Category {
        if inp.is_none() {
            return Category::None
        }

        match inp.unwrap().as_str() {
            "Moderation" => Category::Moderation,
            "Moderation > Name Ban" => Category::ModerationNameBan,
            "Moderation > Discord AutoMod Hook" => Category::ModerationAutoModHook,
            "Moderation > Reports" => Category::ModerationReports,
            "Server Quality of Life" => Category::ServerQoL,
            "Utilities" => Category::Utilities,
            "Utilities > Wiki" => Category::UtilitiesWiki,
            "Leveling" => Category::Leveling,
            "Starboard" => Category::Starboard,
            "Counting" => Category::Counting,
            "Tickets" => Category::Tickets,
            "Tagging" => Category::Tagging,
            "Server Statistics" => Category::ServerStats,
            &_ => Category::None
        }
    }

    fn to_identifier(&self) -> &'static str {
        match self {
            Category::Moderation => "moderation",
            Category::ModerationNameBan => "moderation.nameban",
            Category::ModerationAutoModHook => "moderation.automodhook",
            Category::ModerationReports => "moderation.reports",
            Category::ServerQoL => "serverqol",
            Category::Utilities => "utilities",
            Category::UtilitiesWiki => "utilities.wiki",
            Category::Leveling => "leveling",
            Category::Starboard => "starboard",
            Category::Counting => "counting",
            Category::Tickets => "tickets",
            Category::Tagging => "tagging",
            Category::ServerStats => "serverstats",
            Category::None => ""
        }
    }

    fn from_identifier(inp: Option<String>) -> Category {
        if inp.is_none() {
            return Category::None
        }

        match inp.unwrap().as_str() {
            "moderation" => Category::Moderation,
            "moderation.nameban" => Category::ModerationNameBan,
            "moderation.automodhook" => Category::ModerationAutoModHook,
            "moderation.reports" => Category::ModerationReports,
            "serverqol" => Category::ServerQoL,
            "utilities" => Category::Utilities,
            "utilities.wiki" => Category::UtilitiesWiki,
            "leveling" => Category::Leveling,
            "starboard" => Category::Starboard,
            "counting" => Category::Counting,
            "tickets" => Category::Tickets,
            "tagging" => Category::Tagging,
            "serverstats" => Category::ServerStats,
            &_ => Category::None
        }
    }
}

const CATEGORY: [&str; 13] = [
    "Moderation",
    "Moderation > Name Ban",
    "Moderation > Discord AutoMod Hook",
    "Moderation > Reports",
    "Server Quality of Life",
    "Utilities",
    "Utilities > Wiki",
    "Leveling",
    "Starboard",
    "Counting",
    "Tickets",
    "Tagging",
    "Server Statistics"
];

/// Configures the bot on the server.
#[poise::command(
    slash_command,
    prefix_command,
    category="GuildConfig",
    required_permissions = "MANAGE_GUILD",
    default_member_permissions = "MANAGE_GUILD",
    required_bot_permissions = "MANAGE_GUILD | SEND_MESSAGES | EMBED_LINKS",
    guild_only,
    aliases("guildconfig", "gcfg", "cfg", "serverconfig", "srvcfg"),
    identifying_name = "jUtils.config.guild.json",
)]
pub async fn config(
    ctx: Context<'_>,
    category: Category
) -> Result<(), Error> {
    _ = ctx.defer().await;

    let guild_id = ctx.guild_id().unwrap().to_string();
    if get_guild_config(guild_id.clone()).await.is_err() {
        let _ = create_new_config(guild_id).await;
    }

    let conf = get_config(
        category,
        ctx
    ).await;
    ctx.send(poise::CreateReply::default()
        .embed(conf.0)
        .components(conf.1)
        .reply(true)
        .allowed_mentions(am::new().all_roles(false).all_users(false).everyone(false))
    ).await?;

    config_cmp(ctx).await;
    Ok::<(), Error>(())
}

async fn get_config(category: Category, ctx: Context<'_>) -> (CreateEmbed, Vec<CreateActionRow>) {
    let mut opt: Vec<CreateSelectMenuOption> = Vec::<CreateSelectMenuOption>::new();
    let conf = get_guild_config(ctx.guild_id().unwrap().to_string()).await.expect("Expected GuildConfig");

    for cat in CATEGORY {
        let parts: Vec<&str>;
        if conf.get_status_by_category(Category::from_friendly_name(Some(cat.to_owned()))) {
            let n = &CONFIG.emoji.check_box;
            let cleaned_input = n.trim_start_matches("<:").trim_end_matches(">");
            parts = cleaned_input.split(':').collect();
        } else {
            let n = &CONFIG.emoji.cross_box;
            let cleaned_input = n.trim_start_matches("<:").trim_end_matches(">");
            parts = cleaned_input.split(':').collect();
        }

        opt.push(
            CreateSelectMenuOption::new(
                cat,
                Category::from_friendly_name(Some(cat.to_owned())).to_identifier()
            ).emoji(
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId::from_str(parts[1]).unwrap(),
                    name: Some(parts[0].to_owned())
                }
            )
        )
    }

    let mut embed = CreateEmbed::primary()
        .title(format!("Config for {}", ctx.guild().unwrap().name))
        .description(format!("**{}**", category.to_friendly_name()));
    let selmecat = CreateSelectMenu::new(
        format!("jutils.gconfig.selmenu.{}.category", ctx.id()),
        CreateSelectMenuKind::String { options: opt }
    )
        .max_values(1)
        .min_values(1)
        .placeholder(if category == Category::None {
            "Select a category"
        } else {
            category.to_friendly_name()
        });
    let mut actr = Vec::<CreateActionRow>::new();
    actr.push(CreateActionRow::SelectMenu(selmecat));

    match category {
        Category::Moderation => {
            embed = embed.clone()
                .field("Enabled", conf.modules.moderation.enabled.to_string(), true);
        }
        Category::ModerationAutoModHook => {
            embed = embed.clone()
                .field("Enabled", conf.modules.moderation.automodhook.enabled.to_string(), true)
                .field("Rules", format!("{} rules", conf.modules.moderation.automodhook.rules.len()), true);
        }
        Category::ModerationNameBan => {
            embed = embed.clone()
                .field("Enabled", conf.modules.moderation.nameban.enabled.to_string(), true)
                .field("Action", conf.modules.moderation.nameban.action.to_string(), true)
                .field("Reason", conf.modules.moderation.nameban.reason, true)
                .field("Banned Names", conf.modules.moderation.nameban.banned_names.join(", "), false);
        }
        Category::Counting => {
            let selmechn = CreateSelectMenu::new(
                format!("jutils.gconfig.selmenu.{}.counting.channel", ctx.id()),
                CreateSelectMenuKind::Channel {
                    channel_types: Some(vec![ ChannelType::Text ]),
                    default_channels: None
                }
            )
                .max_values(1)
                .min_values(1)
                .placeholder("Set a channel");
            embed = embed.clone()
                .field("Enabled", conf.modules.counting.enabled.to_string(), true)
                .field("Counting Mode", conf.modules.counting.mode.to_string(), true)
                .field("Channel", format!("<#{}>", conf.modules.counting.channel), true);
            actr.push(
                CreateActionRow::Buttons(
                    vec![
                        CreateButton::new(format!("jutils.gconfig.button.{}.counting.toggle", ctx.id()))
                            .label(
                                if conf.modules.counting.enabled {
                                    "Disable"
                                } else {
                                    "Enable"
                                }
                            ),
                        CreateButton::new(format!("jutils.gconfig.button.{}.counting.toggle_mode", ctx.id()))
                            .label(format!("{}", conf.modules.counting.mode.to_string()))
                    ]
                )
            );
            actr.push(CreateActionRow::SelectMenu(selmechn));
        }
        _ => {}
    };

    (embed, actr)
}

pub async fn config_cmp(ctx: Context<'_>) {
    let prefix_selmenu = format!("jutils.gconfig.selmenu.{}", ctx.id());
    let prefix_button = format!("jutils.gconfig.button.{}", ctx.id());
    let category_selmenu_id = format!("{}.category", prefix_selmenu.clone());
    while let Some(mut press) = {
        let prefix_selmenu_clone = prefix_selmenu.clone();
        let prefix_button_clone = prefix_button.clone();
        serenity::collector::ComponentInteractionCollector::new(ctx)
            .filter(move |press|
                press.data.custom_id.starts_with(prefix_selmenu_clone.as_str()) ||
                press.data.custom_id.starts_with(prefix_button_clone.as_str())
            )
            .timeout(std::time::Duration::from_secs(3600 * 24))
            .await
    } {
        _ = press.defer(ctx.http()).await;
        if press.clone().data.custom_id == category_selmenu_id {
            if let ComponentInteractionDataKind::StringSelect { values } = press.clone().data.kind {
                let conf = get_config(
                    Category::from_identifier(Some(values[0].clone())),
                    ctx
                ).await;

                match press
                    .message
                    .edit(
                        ctx.http(), 
                        EditMessage::new()
                            .embed(conf.0)
                            .components(conf.1)
                    )
                    .await {
                        Err(e) => {
                            panic!("Can't update message. {}", e)
                        }
                        _ => {}
                    };
            };
        }
    }
}