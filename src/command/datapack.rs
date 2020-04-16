use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult};
use serenity::prelude::{Context};
use serenity::framework::standard::macros::{command};
use serenity::utils::MessageBuilder;
use log::{info, debug};
use crate::VerifyChannel;
use crate::utils::*;

#[command]
#[aliases("verify")]
#[description("Request datapack verification from admin")]
#[usage("<datapack url>")]
#[num_args(1)]
#[only_in(guilds)]
#[checks(is_in_verify_channel)]
fn request_verification(context: &mut Context, message: &Message) -> CommandResult {
	info!("{user} invoke `{command}`", user = message.author.tag(), command = message.content);

	if message.pin(&context).is_ok() {
		let response = MessageBuilder::new()
			.push("Your message has been pinned and will be review by Boomber")
			.build();
		message.channel_id.say(&context.http, &response)?;
	} else {
		let response = MessageBuilder::new()
			.push("We cannot pin your message due to pin limit")
			.build();
		message.channel_id.say(&context.http, &response)?;
	}
	
	Ok(())
}

#[command]
#[aliases("verify_channels")]
#[description("Get a list of verification channels inside this guild")]
#[num_args(0)]
#[only_in(guilds)]
fn list_verify_channel(context: &mut Context, message: &Message) -> CommandResult {
	info!("{user} invoke `{command}`", user = message.author.tag(), command = message.content);
	let guild_id = message.guild_id.expect("Guild ID not found");

	let data = context.data.read();
	let verify_channel = data.get::<VerifyChannel>().expect("VerifyChannel does not exists");

	if let Some(channels) = verify_channel.get(&guild_id) {
		let mut response = MessageBuilder::new();
		response.push("Found: ");

		for channel in channels {
			response.channel(channel);
			response.push(" ");
		}

		let response = response.build();
		message.channel_id.say(&context.http, &response)?;
	} else {
		debug!("This guild doesn't have verification channel");
		let response = MessageBuilder::new()
			.push("No verification channels found")
			.build();

		message.channel_id.say(&context.http, &response)?;
	}
	
	Ok(())
}
