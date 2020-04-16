use log::{debug, info};
use serenity::builder::EditRole;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::prelude::{Role, RoleId, UserId};
use serenity::model::Permissions;
use serenity::utils::{MessageBuilder};
use serenity::prelude::Context;
use random_color::RandomColor;

#[command]
#[required_permissions(ADMINISTRATOR)]
#[min_args(2)]
#[only_in(guilds)]
fn give_role(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
	info!("{user} invoke `{command}`", user = message.author.tag(), command = message.content);

	let user = args.single::<UserId>()?;
	let roles: Result<Vec<String>, _> = args.iter::<String>().collect();
	let roles = roles?;

	if let Some(guild) = message.guild(&context) {
		let guild = guild.write();

		let applied_roles: Result<Vec<Role>, _> = roles
			.iter()
			.map(|name| {
				if let Some(role) = guild.role_by_name(name) {
					Ok(role.clone())
				} else {
					guild.create_role(&context.http, |role| role_creator(role, name))
				}
			})
			.collect();
		let applied_roles = applied_roles?;
		let displayed_role: Vec<String> = applied_roles.clone().iter().map(|x| x.name.to_owned()).collect();
		let displayed_role = displayed_role.join(", ");
		let applied_roles: Vec<RoleId> = applied_roles.into_iter().map(|v| v.id).collect();

		let mut member = guild.member(&context.http, user)?;
		member.add_roles(&context.http, &applied_roles)?;

		let response = MessageBuilder::new()
			.push("Added these roles: ")
			.push(displayed_role)
			.push(" to ")
			.user(member)
			.build();
		message.channel_id.say(&context.http, &response)?;
	}

	Ok(())
}

fn role_creator<'a>(role: &'a mut EditRole, name: &str) -> &'a mut EditRole {
	let [r, g, b] = RandomColor::new().to_rgb_array();
	let color = r as u64 * 0x010_000 + g as u64 * 0x000_100 + b as u64;

	role
		.name(name)
		.permissions(Permissions::empty())
		.colour(color)
}
