use dotenv::dotenv;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::EventHandler;
use serenity::Client;
use std::collections::HashMap;
use std::env;

mod command;
mod data;
mod help;
mod utils;

use command::*;
use data::*;
use help::*;

struct Handler;

impl EventHandler for Handler {}

fn main() {
	dotenv().expect("Fail to load .env file");

	env_logger::init();

	let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN in environment variable");
	let mut client = Client::new(&token, Handler).expect("Error creating client");
	client.with_framework(
		StandardFramework::new()
			.configure(|c| c.prefix("!"))
			.group(&DATAPACK_GROUP)
			.group(&ADMIN_GROUP)
			.group(&MEGUMIN_GROUP)
			.help(&HELP_MESSAGE),
	);

	{
		let mut data = client.data.write();
		data.insert::<VerifyChannel>(HashMap::default());
	}

	if let Err(error) = client.start() {
		println!("Error: {}", error);
	}
}

#[group]
#[description = "Datapack-related command group"]
#[commands(request_verification, list_verify_channel)]
struct Datapack;

#[group]
#[description = "Admin command group"]
#[commands(set_verification_channel, clear_verification_channel)]
#[required_permissions(ADMINISTRATOR)]
struct Admin;

#[group]
#[description = "Megumin's command group"]
#[commands(give_role)]
struct Megumin;
