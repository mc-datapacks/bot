use crate::data::{Error, RoleRequest};
use crate::prelude::*;
use crate::trading::Trade;
use crate::utils::*;
use crate::{bail, report};
use log::info;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};

#[group]
#[description = "Vanity command group"]
#[prefix("megu")]
#[commands(give_role, stats, request, accept, cancel)]
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

#[command]
#[description = "Request a trade"]
#[only_in(guilds)]
#[min_args(2)]
fn request(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    invoke_command(message);
    let id = message.guild_id.ok_or_else(|| Error::OutsideGuild)?;

    let target = args.single::<UserId>()?;
    let role_name = args.rest();

    let result = role_by_name(role_name, context, message);
    let role = report!(result => context => message);

    let target_user = target.to_user(&context.http)?;
    let request_user = &message.author;

    let tg_state = target_user.has_role(&context.http, id, role.id)?;
    let rq_state = request_user.has_role(&context.http, id, role.id)?;

    if tg_state && rq_state {
        bail!("Both account already have this role" => context => message);
    }

    if !tg_state {
        bail!("Targeted user doesn't have this role" => context => message);
    }

    let mut data = context.data.write();
    if let Some(manager) = data.get_mut::<RoleRequest>() {
        let trade = Trade::create(request_user.id, target_user.id, role.id);
        report!(manager.create(trade) => context => message);
        let content = MessageBuilder::new()
            .push("Requesting the role ")
            .push(format!("'{}'", role_name))
            .push(" from ")
            .user(target_user.id)
            .build();

        message.channel_id.say(&context.http, content)?;
    }

    Ok(())
}

#[command]
#[description = "Accept the offer"]
#[only_in(guilds)]
#[num_args(1)]
fn accept(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    invoke_command(message);

    let user = args.single::<UserId>()?;

    let target = &message.author;
    let requester = user.to_user(&context.http)?;

    let guild = message
        .guild(&context.cache)
        .ok_or_else(|| Error::OutsideGuild)?;

    let mut data = context.data.write();
    if let Some(manager) = data.get_mut::<RoleRequest>() {
        let result = manager.accept(requester.id, target.id);
        let role = report!(result => context => message);

        let guild = guild.write();
        let mut member = guild.member(&context.http, target.id)?;
        member.remove_role(&context.http, role)?;

        let mut member = guild.member(&context.http, requester.id)?;
        member.add_role(&context.http, role)?;

        message.channel_id.say(&context.http, "Trade success!")?;
    }

    Ok(())
}

#[command]
#[description = "Cancel all requests"]
#[only_in(guilds)]
fn cancel(context: &mut Context, message: &Message) -> CommandResult {
    invoke_command(message);

    let mut data = context.data.write();
    if let Some(manager) = data.get_mut::<RoleRequest>() {
        manager.clear(message.author.id);
        message
            .channel_id
            .say(&context.http, "Clear all requests made by you")?;
    }

    Ok(())
}
