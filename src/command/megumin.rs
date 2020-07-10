use crate::data::Error;
use crate::prelude::*;
use crate::utils::*;
use log::info;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};

#[group]
#[description = "Vanity command group"]
#[prefix("megu")]
#[commands(give_role, stats)]
pub struct Megumin;

#[command("role")]
#[description = "Create and give role to the user"]
#[required_permissions(ADMINISTRATOR)]
#[min_args(2)]
#[only_in(guilds)]
fn give_role(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    invoke_command(&message);

    let user = args.single::<UserId>()?;
    info!("Get user id: {}", user);
    let role_name = args.rest();

    // Discord really doesn't like empty role
    if role_name.is_empty() {
        info!("Role name is empty");
        return Err(Error::EmptyRoleName.into());
    }

    let role = create_role(context, message, role_name)?;
    let member = apply_role(context, message, user, &role)?;

    let response = MessageBuilder::new()
        .push("Added role '")
        .push(role.name)
        .push("' to ")
        .user(member)
        .build();
    message.channel_id.say(&context, response)?;
    Ok(())
}

fn create_role(context: &Context, message: &Message, role_name: &str) -> Result<Role, Error> {
    get_guild(context, message, |guild| {
        let guild = guild.read();
        let role_query = guild.role_by_name(role_name);

        match role_query {
            Some(role) => Ok(role.clone()),
            None => guild
                .create_role(&context.http, |edit| role_creator(edit, role_name))
                .map_err(|err| err.into()),
        }
    })
}

pub fn apply_role(
    context: &Context,
    message: &Message,
    user: UserId,
    role: &Role,
) -> Result<Member, Error> {
    get_guild(context, message, |guild| {
        let guild = guild.read();
        let mut member = guild.member(&context.http, user)?;
        member.add_role(&context.http, role)?;
        Ok(member)
    })
}

#[command]
#[description = "Display server stats"]
#[only_in(guilds)]
fn stats(context: &mut Context, message: &Message) -> CommandResult {
    invoke_command(&message);

    let name = server_name(context, message)?;
    let description = stats_format(context, message)?;

    message.channel_id.send_message(&context.http, |message| {
        message.embed(|embed| embed.title(name).description(description))
    })?;

    Ok(())
}

pub fn server_name(context: &Context, message: &Message) -> Result<String, Error> {
    get_guild(context, message, |guild| {
        let guild = guild.read();
        let result = format!("{}'s Server information", guild.name);
        Ok(result)
    })
}

pub fn stats_format(context: &Context, message: &Message) -> Result<String, Error> {
    get_guild(context, message, |guild| {
        let guild = guild.read();

        let members = guild.member_count;
        let channels = guild.channels.len();
        let roles = guild.roles.len();
        let emojis = guild.emojis.len();

        let result = format!(
            "\
		This server contains...\n\
		- {0} members.\n\
		- {1} channels.\n\
		- {2} roles.\n\
		- {3} custom emotes.\
		",
            members, channels, roles, emojis
        );

        Ok(result)
    })
}
