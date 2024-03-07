use chrono::TimeDelta;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateAllowedMentions as am;
use poise::serenity_prelude::PartialGuild;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;
use ::serenity::all::CreateActionRow;
use ::serenity::all::CreateButton;
use ::serenity::all::CreateEmbed;
use ::serenity::all::Member;
use ::serenity::all::Permissions;
use std::time::Duration;
use crate::EmbedHelper;
use crate::CONFIG;
use poise::serenity_prelude::Context;
use poise::serenity_prelude::UserId;
use poise::Command;
use chrono::DateTime;
use chrono::Utc;

fn get_fmt_params<U, E>(cmd: &Command<U, E>) -> String {
    let mut str: Vec<String> = Vec::<String>::new();

    let pars = &cmd.parameters;

    for par in pars {
        str.push(
            if par.required {
                format!("<{}>", par.name.clone())
            } else {
                format!("[{}]", par.name)
            }
        )
    }

    str.join(" ")
}

pub async fn get_command<U, E>(
    ctx: poise::Context<'_, U, E>,
    cmd: String
) -> Result<CreateEmbed, serenity::Error> {
    let mut commands = Vec::<&Command<U, E>>::new();
    for cmd in &ctx.framework().options().commands {
        commands
            .push(cmd);
    }

    let mut scmd: Option<&Command<U, E>> = None;

    let embed: CreateEmbed;
    for command in commands {
        if command.hide_in_help {
            continue
        }
        
        if command.name == cmd || command.aliases.contains(&cmd) {
            scmd = Some(command);
        }
    }

    embed = match scmd {
        Some(c) => {
            let mut em = CreateEmbed::primary()
                .title(c.name.clone())
                .description(c.description.clone().unwrap_or("No description".to_owned()))
                .field("Usage", format!("`{}{} {}`", ctx.prefix(), c.name, get_fmt_params(c)), false)
                .field("Category", c.category.clone().unwrap_or("No category".to_owned()), true)
                .field("Aliases", c.aliases.join(", "), true);

            if !c.required_permissions.is_empty() {
                em = em.clone().field("Requires", c.required_permissions.iter_names().map(|name| name.0).collect::<Vec<&str>>().join(", "), true);
            }

            em
        },
        None => {
            CreateEmbed::error()
                .title("Cannot find command!")
                .description("Make sure that the command is correct!")
        }
    };

    Ok(embed)
}

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

    let mut categories = std::collections::HashMap::new();

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

    Ok(chunk(menu, 25))
}

pub fn chunk<T: Clone>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    vec.into_iter()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>()
}

pub fn format_duration(duration: Duration) -> String {
    let mut duration_secs = duration.as_secs();
    let days = duration_secs / (24 * 3600);
    duration_secs %= 24 * 3600;
    let hours = duration_secs / 3600;
    duration_secs %= 3600;
    let minutes = duration_secs / 60;
    let seconds = duration_secs % 60;

    let mut result = String::new();
    if days > 0 {
        result.push_str(&format!("{:02}:", days));
    }
    if hours > 0 || days > 0 {
        result.push_str(&format!("{:02}:", hours));
    }
    result.push_str(&format!("{:02}:{:02}", minutes, seconds));

    result
}

pub fn duration_to_rfc3339(duration: Duration) -> String {
    let now: DateTime<Utc> = Utc::now();
    let datetime = now + duration;
    datetime.to_rfc3339()
}

pub fn duration_to_datetime(duration: std::time::Duration) -> chrono::DateTime<Utc> {
    let now: DateTime<Utc> = Utc::now();
    now + duration
}

pub async fn compare_roles(ctx: &Context, guild: PartialGuild, user_id: UserId) -> bool {
    let cu = ctx.http.get_current_user().await.expect("Expected a current user.");
    let bot_member = match guild.member(ctx.http.clone(), cu.id).await {
        Ok(member) => member,
        Err(_) => return false
    };
    let user_member = match guild.member(ctx.http.clone(), user_id).await {
        Ok(member) => member,
        Err(_) => return false,
    };

    let bhighest_role = bot_member.highest_role_info(ctx.cache.clone()).expect("Expected roles");
    let uhighest_role = user_member.highest_role_info(ctx.cache.clone()).expect("Expected roles");

    if bhighest_role.1 < uhighest_role.1 {
        return true
    }
    false
}

pub async fn fetch_package_info(base_url: &str, package_name: &str, suffix: &str) -> Result<serde_json::Value, serenity::Error> {
    let client = reqwest::Client::builder()
        .user_agent("Your User-Agent Here")
        .build()?;

    let response = client.get(&format!("{}{}{}", base_url, package_name, suffix))
        .send()
        .await?;

    if response.status().is_success() {
        let data = response.json::<serde_json::Value>().await?;
        Ok(data)
    } else {
        let t = match response.text().await {
			Ok(s) => s,
			Err(_) => "".to_owned()
		};
        println!("{}", t.clone());
        Err(serenity::Error::Other(Box::leak(t.into_boxed_str())))
    }
}

pub async fn send_package_info<'a, U, E>(ctx: poise::Context<'a, U, E>, package_info: serde_json::Value, package_name: &'a str) -> Result<(), serenity::Error> {
    let mut components = Vec::<CreateActionRow>::new();
    let mut btns = Vec::<CreateButton>::new();

    let created_at = DateTime::parse_from_rfc3339(package_info["created_at"].as_str().expect("Expected created at")).unwrap();
    let created_at_us = created_at.timestamp();

    let lv_published_at = DateTime::parse_from_rfc3339(package_info["versions"][0]["created_at"].as_str().expect("Expected created at")).unwrap();
    let lv_published_at_us = lv_published_at.timestamp();

    let embed = CreateEmbed::primary()
        .title(format!("Package {}", package_name))
        .description(package_info["description"].as_str().unwrap_or("No description available"))
        .field("Published at", format!("<t:{}>", created_at_us), true)
        .field("Downloads", package_info["downloads"].as_u64().unwrap_or(0).to_string(), true)
        .field("Latest Version", format!("{:?}\nPublished at <t:{:?}>\n{:?} downloads",
            package_info["versions"][0]["num"].as_str(),
            lv_published_at_us,
            package_info["versions"][0]["downloads"].clone().take().as_u64()), true);

    if let Some(documentation) = package_info["documentation"].as_str() {
        btns.push(CreateButton::new_link(documentation).label("Documentation"))
    }
    if let Some(homepage) = package_info["homepage"].as_str() {
        btns.push(CreateButton::new_link(homepage).label("Homepage"))
    }
    if let Some(repository) = package_info["repository"].as_str() {
        btns.push(CreateButton::new_link(repository).label("Repository"))
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

    Ok(())
}

pub fn generate_id(num: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(num)
        .map(char::from)
        .collect()
}