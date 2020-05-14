use crate::utils::*;
use crate::{data::Error, VerifyChannel};
use log::{debug, info};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::{id::ChannelId, channel::Message};
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;
use std::collections::HashSet;

#[command]
#[aliases("verify")]
#[description("Request datapack verification from admin")]
#[usage("<datapack url>")]
#[num_args(1)]
#[only_in(guilds)]
#[checks(is_in_verify_channel)]
fn request_verification(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
	info!("{user} invoke `{command}`", user = message.author.tag(), command = message.content);

	let url = args.single::<String>()?;
	if url.is_empty() {
		let response = MessageBuilder::new()
			.push("You forgot your URL, you dummy >///<")
			.build();
		message.channel_id.say(context, response)?;

		return Ok(())
	}

	if message.pin(&context).is_ok() {
		let response = MessageBuilder::new()
			.push("Your message has been pinned and will be review by Boomber")
			.build();
		message.channel_id.say(context, &response)?;
	} else {
		let response = MessageBuilder::new()
			.push("We cannot pin your message due to pin limit")
			.build();
		message.channel_id.say(context, &response)?;
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
	let guild = message.guild(&context).ok_or(Error::OutsideGuild)?;
	let guild = guild.read();

	let channels: HashSet<ChannelId> = guild.channels(&context)?.keys().copied().collect();

	let data = context.data.read();
	let database = data.get::<VerifyChannel>().ok_or(Error::MissingDatabase)?;

	let channels = database.intersect(&channels);

	if channels.is_empty() {
		debug!("This guild doesn't have verification channel");
		let response = MessageBuilder::new()
			.push("No verification channels found")
			.build();

		message.channel_id.say(&context, &response)?;
	} else {
		debug!("Found: {:?}", channels);
		let mut response = MessageBuilder::new();
		response.push("Found: ");

		channels.iter().for_each(|&channel| {
			response.channel(channel);
			response.push(" ");
		});

		let response = response.build();
		message.channel_id.say(&context, &response)?;
	}

	Ok(())
}
