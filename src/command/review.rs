use crate::prelude::*;
use crate::utils::{get_database, invoke_command};
use std::collections::HashSet;

#[group]
#[prefix = "review"]
#[description = "Review command group"]
#[commands(add, remove, clear, list)]
#[default_command(list)]
pub struct Review;

#[command]
#[description = "Add review channel"]
#[usage = "#channel-mention"]
#[required_permissions(ADMINISTRATOR)]
#[only_in(guilds)]
pub fn add(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    invoke_command(&message);

    let channel = args.single::<ChannelId>()?;
    get_database(context, |database| database.add_channel(channel))?;

    let response = MessageBuilder::new()
        .push("Added ")
        .channel(channel)
        .push(" to review channel list")
        .build();
    message.channel_id.say(&context, response)?;
    Ok(())
}

#[command]
#[description = "Remove review channel"]
#[usage = "#channel-mention"]
#[required_permissions(ADMINISTRATOR)]
#[only_in(guilds)]
pub fn remove(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    invoke_command(&message);

    let channel = args.single::<ChannelId>()?;
    get_database(context, |database| database.remove_channel(channel))?;

    let response = MessageBuilder::new()
        .push("Removed ")
        .channel(channel)
        .push(" from review channel list")
        .build();
    message.channel_id.say(&context, response)?;
    Ok(())
}

#[command]
#[description = "Clear all review channels in this guild"]
#[num_args(0)]
#[required_permissions(ADMINISTRATOR)]
#[only_in(guilds)]
pub fn clear(context: &mut Context, message: &Message) -> CommandResult {
    invoke_command(&message);

    let channels = channels_from_guild(context, message).ok_or(Error::OutsideGuild)?;
    get_database(context, move |database| database.remove_channels(channels))?;

    let response = MessageBuilder::new()
        .push("Clear all review channels from this guild.")
        .build();
    message.channel_id.say(&context, response)?;
    Ok(())
}

pub fn channels_from_guild(context: &Context, message: &Message) -> Option<HashSet<ChannelId>> {
    let guild = message.guild(context)?;
    let guild = guild.read();
    let result = guild.channels.keys().cloned().collect();
    Some(result)
}

#[command]
#[description = "List all review channels in this guild"]
#[num_args(0)]
#[required_permissions(ADMINISTRATOR)]
#[only_in(guilds)]
pub fn list(context: &mut Context, message: &Message) -> CommandResult {
    invoke_command(&message);

    let channels = channels_from_guild(context, message).ok_or(Error::OutsideGuild)?;
    let response: Result<String, Error> = get_database(context, move |database| {
        let intersect = database.intersect(&channels);

        let response = if intersect.is_empty() {
            String::from("There is no review channel in this guild.")
        } else {
            let mut response = MessageBuilder::new();
            response.push("Found:");
            intersect.iter().for_each(|&channel| {
                response.push(" ");
                response.channel(channel);
            });
            response.build()
        };

        Ok(response)
    });

    message.channel_id.say(&context, &response?)?;
    Ok(())
}
