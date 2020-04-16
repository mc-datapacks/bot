use serenity::model::prelude::UserId;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult};
use serenity::prelude::{Context};
use serenity::framework::standard::macros::{help};
use serenity::framework::standard::{help_commands, Args, HelpOptions, CommandGroup};
use std::collections::HashSet;

#[help]
#[strikethrough_commands_tip_in_guild = ""]
#[lacking_permissions = "Hide"]
fn help_message(ctx: &mut Context, msg: &Message, args: Args, help_options: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult {
	help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
}