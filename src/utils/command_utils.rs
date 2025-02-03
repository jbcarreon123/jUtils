use poise::serenity_prelude as serenity;
use poise::Command;
use ::serenity::all::CreateEmbed;

use crate::types::EmbedHelper;

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
