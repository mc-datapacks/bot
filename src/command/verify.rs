use crate::prelude::*;
use crate::utils::*;

#[group]
#[description = "Verify command group"]
#[commands(verify)]
pub struct Verify;

#[command]
#[description = "Request datapack verification from reviewer"]
#[usage = "<url-to-datapack>"]
#[only_in(guilds)]
#[checks(is_in_verify_channel)]
pub fn verify(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    invoke_command(&message);

    let url = args.single::<String>()?;
    if url.is_empty() {
        let response = String::from("You forgot your URL >///<");
        message.channel_id.say(&context, response)?;
        return Ok(());
    }

    let response = match message.pin(&context) {
        Ok(_) => String::from("Your message has been pinned"),
        Err(err) => format!(
            "Unable to pin your message with the following error: {}",
            err
        ),
    };

    message.channel_id.say(&context, &response)?;
    Ok(())
}
