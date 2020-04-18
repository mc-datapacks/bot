use crate::VerifyChannel;
use log::{debug, info};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;

#[command]
#[aliases("verify_channel")]
#[help_available]
#[description("Set the channel to accept verification request")]
#[usage("<channel id>")]
#[num_args(1)]
#[required_permissions(ADMINISTRATOR)]
#[only_in(guilds)]
fn set_verification_channel(
	context: &mut Context,
	message: &Message,
	mut args: Args,
) -> CommandResult {
	info!(
		"{user} invoke `{command}`",
		user = message.author.tag(),
		command = message.content
	);

	let channel = args.single::<ChannelId>()?;

	let mut data = context.data.write();
	if let Some(verify_channel) = data.get_mut::<VerifyChannel>() {
		let guild_id = message.guild_id.expect("Guild ID not found");

		debug!(
			"Add {} channel from {} guild to verify channel list",
			channel, guild_id
		);
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
	info!(
		"{user} invoke `{command}`",
		user = message.author.tag(),
		command = message.content
	);

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
