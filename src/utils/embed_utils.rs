use poise::serenity_prelude as serenity;
use poise::Command;
use ::serenity::all::Permissions;
use std::collections::HashMap;
use crate::CONFIG;

#[allow(suspicious_double_ref_op)]
pub async fn get_perms_as_embedfields<U, E>(
    ctx: poise::Context<'_, U, E>
) -> Result<Vec<(String, String, bool)>, serenity::Error> {
    let cu = ctx.serenity_context().http.get_current_user().await.expect("Expected a current user.");
    let guild = ctx.http().get_guild(ctx.guild_id().expect("Expected GuildId")).await.expect("Expected Guild");
    let cug = match guild.member(ctx.serenity_context().http.clone(), cu.id).await {
        Ok(member) => member,
        Err(e) => return Err(e)
    };

    let mut commands = Vec::<&Command<U, E>>::new();
    for cmd in &ctx.framework().options().commands {
        if cmd.hide_in_help {
            continue
        }

        commands
            .push(cmd);
    }

    let mut menu = Vec::<(String, String, bool)>::new();

    let mut categories = HashMap::new();

    for item in commands.clone() {
        categories
            .entry(item.category.clone())
            .or_insert_with(Vec::new)
            .push(item);
    }

    for category in categories {
        let catcmds: Vec<&Command<U, E>> = commands
            .iter()
            .filter(|x| x.category.clone().unwrap_or("No category".to_owned()) == category.0.clone().unwrap_or("No category".to_owned()))
            .map(|x| x.clone())
            .collect();

        let mut perms: Permissions = Permissions::default();
        for c in catcmds {
            for p in c.required_bot_permissions {
                perms.insert(p);
            }
        }

        let mut prmstr = "".to_owned();
        let cugprm = cug.permissions(ctx.cache()).unwrap();
        for perm in perms {
            prmstr += &format!(
                "{} {}",
                if cugprm.contains(perm) {
                    CONFIG.emoji.check_box.clone()
                } else {
                    CONFIG.emoji.cross_box.clone()
                },
                perm.get_permission_names().join(" ")
            );
            prmstr += "\n";
        }

        if prmstr.is_empty() {
            continue;
        }

        menu.push((category.0.unwrap_or("No category".to_owned()), prmstr, true))
    }

    Ok(menu)
}

pub async fn get_all_commands_as_embedfields<U, E>(
    ctx: poise::Context<'_, U, E>
) -> Result<Vec<Vec<(String, String, bool)>>, serenity::Error> {
    let mut commands = Vec::<&Command<U, E>>::new();
    for cmd in &ctx.framework().options().commands {
        if cmd.hide_in_help {
            continue
        }

        commands
            .push(cmd);
    }

    let mut menu = Vec::<(String, String, bool)>::new();
    for command in commands {
        let desc: String = match &command.description {
            Some(string) => string.clone(),
            None => "No description".to_owned()
        };
        let cat: String = match &command.category {
            Some(str) => format!("Category: {}\n", str),
            None => "".to_owned()
        };
        let pref = if ctx.prefix().starts_with("<@") {
            CONFIG.discordbot.prefix.clone()
        } else {
            ctx.prefix().to_owned()
        };
        menu.push((
            format!("{}{}", pref, command.name),
            format!("{}{}", cat, desc),
            true
        ))
    }

    Ok(chunk(menu, 9))
}

pub fn chunk<T: Clone>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    vec.into_iter()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>()
}
