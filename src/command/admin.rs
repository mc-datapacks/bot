use crate::{data::Error, VerifyChannel};
use log::info;
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
fn set_verification_channel(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
	info!("{user} invoke `{command}`", user = message.author.tag(), command = message.content);
	let mut data = context.data.write();
	let database = data.get_mut::<VerifyChannel>().ok_or("hello")?;
	
	let channel = args.single::<ChannelId>()?;
	database.add_channel(channel)?;

	let response = MessageBuilder::new()
		.push("Added ")
		.channel(channel)
		.push(" to verification channel list")
		.build();
	message.channel_id.say(&context, response)?;

	Ok(())
}

#[command]
#[aliases("remove_channel")]
#[help_available]
#[description("Remove the verification channel")]
#[usage("<channel id>")]
#[num_args(1)]
#[required_permissions(ADMINISTRATOR)]
#[only_in(guilds)]
fn remove_verification_channel(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
	info!("{user} invoke `{command}`", user = message.author.tag(), command = message.content);
	let mut data = context.data.write();
	let database = data.get_mut::<VerifyChannel>().ok_or("hello")?;
	
	let channel = args.single::<ChannelId>()?;
	database.remove_channel(&channel)?;

	let response = MessageBuilder::new()
		.push("Removed ")
		.channel(channel)
		.push(" from verification channel list")
		.build();
	message.channel_id.say(&context, response)?;

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

	let guild = message.guild(&context).ok_or(Error::OutsideGuild)?;
	let guild = guild.read();

	let channels = guild.channels(&context)?
		.keys()
		.copied()
		.collect();

	let mut data = context.data.write();
	let database = data.get_mut::<VerifyChannel>().ok_or(Error::MissingDatabase)?;

	database.remove_channels(channels)?;

	let response = MessageBuilder::new()
		.push("Clear all verification channels.")
		.build();
	message.channel_id.say(&context, response)?;
	
	Ok(())
}
