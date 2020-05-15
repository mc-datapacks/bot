use log::info;
use random_color::RandomColor;
use serenity::builder::EditRole;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::prelude::UserId;
use serenity::model::Permissions;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;
use crate::data::Error;

#[command]
#[required_permissions(ADMINISTRATOR)]
#[min_args(2)]
#[only_in(guilds)]
fn give_role(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
	info!("{user} invoke `{command}`", user = message.author.tag(), command = message.content);

	let user = args.single::<UserId>()?;
	info!("Get user id: {}", user);
	let role = args.rest();

	// Discord really doesn't like empty role
	if role.is_empty() {
		info!("Role name is empty");
		return Err(Error::EmptyRoleName.into());
	}

	let guild = message.guild(&context).ok_or(Error::OutsideGuild)?;
	let guild = guild.read();
	info!("Get guild's read mutex");

	// Can't fucking refactor this because RwLockWriteGuard is private or hidden somewhere I don't know
	let applied_role = if let Some(role) = guild.role_by_name(&role) {
		Ok(role.clone())
	} else {
		guild.create_role(&context, |edit| role_creator(edit, role))
	};
	let applied_role = applied_role?;
	info!("Get role by name: {}", applied_role.id);
	
	let mut member = guild.member(&context, user)?;
	info!("Get member: {}", member.distinct());
	member.add_role(&context, applied_role.id)?;
	info!("Applied role to member: {}", member.distinct());

	let response = MessageBuilder::new()
		.push("Added role '")
		.push(applied_role.name)
		.push("' to ")
		.user(member)
		.build();
	message.channel_id.say(&context, &response)?;

	Ok(())
}

fn role_creator<'a>(role: &'a mut EditRole, name: &str) -> &'a mut EditRole {
	let [r, g, b] = RandomColor::new().to_rgb_array();
	let color = convert_rgb(r, g, b);

	role.name(name)
		.permissions(Permissions::empty())
		.colour(color)
}

fn convert_rgb(r: u32, g: u32, b: u32) -> u64 {
	(r as u64) << 16 | (g as u64) << 8 | b as u64
}
