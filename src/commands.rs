use std::str::FromStr;
use serenity::model::id::ChannelId;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, Args};
use serenity::prelude::{Context};
use serenity::framework::standard::macros::{command};
use serenity::utils::MessageBuilder;
use log::{info, debug};
use super::VerifyChannel;
use super::utils::*;

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

#[command]
#[aliases("verify_channel")]
#[help_available]
#[description("Set the channel to accept verification request")]
#[usage("<channel id>")]
#[num_args(1)]
#[required_permissions(ADMINISTRATOR)]
#[only_in(guilds)]
fn set_verification_channel(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
	info!("{user} invoke `{command}`", user = message.author.tag(), command = message.content);

	let channel = args.single::<String>()?;
	let channel = ChannelId::from_str(&channel)?;
	
	let mut data = context.data.write();
	if let Some(verify_channel) = data.get_mut::<VerifyChannel>() {
		let guild_id = message.guild_id.expect("Guild ID not found");

		debug!("Add {} channel from {} guild to verify channel list", channel, guild_id);
		match verify_channel.get_mut(&guild_id) {
			Some(v) => v.push(channel),
			None => {
				let value = vec![channel];
				verify_channel.insert(guild_id, value);
			}
		}

		let response = MessageBuilder::new()
			.push("Added ")
			.channel(channel)
			.push(" to verification channel list")
			.build();
		message.channel_id.say(&context.http, &response)?;
	}

	Ok(())
}

#[command]
#[aliases("explosion!")]
#[help_available]
#[description("Clear verification channels list from this guild")]
#[num_args(0)]
#[required_permissions(ADMINISTRATOR)]
#[only_in(guilds)]
fn clear_verification_channel(context: &mut Context, message: &Message) -> CommandResult {
	info!("{user} invoke `{command}`", user = message.author.tag(), command = message.content);
	
	let mut data = context.data.write();
	if let Some(verify_channel) = data.get_mut::<VerifyChannel>() {
		let guild_id = message.guild_id.expect("Guild ID not found");

		debug!("Clear verification channels list from {} guild", guild_id);
		
		verify_channel.remove(&guild_id);

		let response = MessageBuilder::new()
			.push("Clear all verification channels.")
			.build();
		message.channel_id.say(&context.http, &response)?;
	}

	Ok(())
}