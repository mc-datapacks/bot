use super::VerifyChannel;
use serenity::framework::standard::macros::check;
use serenity::framework::standard::CheckResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[check]
#[name = "is_in_verify_channel"]
pub fn is_in_verify_channel(context: &mut Context, message: &Message) -> CheckResult {
	let data = context.data.read();

	if let Some(database) = data.get::<VerifyChannel>() {
		if database.exists(&message.channel_id) {
			return CheckResult::Success;
		}
	}

	CheckResult::new_user("You're not in a verification channel")
}
