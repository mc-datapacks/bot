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

#[command]
#[required_permissions(ADMINISTRATOR)]
#[min_args(2)]
#[only_in(guilds)]
fn give_role(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
	info!(
		"{user} invoke `{command}`",
		user = message.author.tag(),
		command = message.content
	);

	let user = args.single::<UserId>()?;
	let role: Result<String, _> = args.iter::<String>().collect();
	let role = role?;

	// Discord really doesn't like empty role
	if role.is_empty() {
		return Ok(());
	}

	if let Some(guild) = message.guild(&context) {
		let guild = guild.write();

		let applied_role = if let Some(role) = guild.role_by_name(&role) {
			Ok(role.clone())
		} else {
			guild.create_role(&context.http, |r| role_creator(r, &role))
		};

		let applied_role = applied_role?;

		let mut member = guild.member(&context.http, user)?;
		member.add_role(&context.http, applied_role.id)?;

		let response = MessageBuilder::new()
			.push("Added role '")
			.push(applied_role.name)
			.push("' to ")
			.user(member)
			.build();
		message.channel_id.say(&context.http, &response)?;
	}

	Ok(())
}

fn role_creator<'a>(role: &'a mut EditRole, name: &str) -> &'a mut EditRole {
	let [r, g, b] = RandomColor::new().to_rgb_array();
	let color = r as u64 * 0x010_000 + g as u64 * 0x000_100 + b as u64;

	role.name(name)
		.permissions(Permissions::empty())
		.colour(color)
}
