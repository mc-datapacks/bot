use super::VerifyChannel;
use serenity::framework::standard::macros::check;
use serenity::framework::standard::CheckResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[check]
#[name = "is_in_verify_channel"]
pub fn is_in_verify_channel(context: &mut Context, message: &Message) -> CheckResult {
	let guild_id = message.guild_id.expect("Guild ID not found");
	let data = context.data.read();

	if let Some(verify_channel) = data.get::<VerifyChannel>() {
		if let Some(channels) = verify_channel.get(&guild_id) {
			let current_channel = message.channel_id;

			if channels.contains(&current_channel) {
				return CheckResult::Success;
			}
		}
	}

	CheckResult::new_user("You're not in a verification channel")
}
