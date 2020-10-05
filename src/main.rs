use dotenv::dotenv;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::EventHandler;
use serenity::Client;
use std::env;

mod command;
mod data;
mod database;
mod help;
mod trading;
mod utils;

use command::*;
use data::*;
use database::Database;
use help::*;
use trading::RequestDatabase;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    dotenv().expect("Fail to load .env file");
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN in environment variable");
    let mut client = Client::new(&token, Handler).expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!").allow_dm(false))
            .group(&review::REVIEW_GROUP)
            .group(&verify::VERIFY_GROUP)
            .group(&megumin::MEGUMIN_GROUP)
            .help(&HELP_MESSAGE),
    );

    {
        let database = Database::new("database.db").expect("Unable to create database");
        let trade = RequestDatabase::new();

        let mut data = client.data.write();
        data.insert::<VerifyChannel>(database);
        data.insert::<RoleRequest>(trade);
    }

    if let Err(error) = client.start() {
        println!("Error: {}", error);
    }
}

pub mod prelude {
    pub use crate::data::Error;
    pub use serenity::framework::standard::macros::{check, command, group, help};
    pub use serenity::framework::standard::{Args, CommandResult};
    pub use serenity::model::channel::Message;
    pub use serenity::model::guild::{Guild, Member};
    pub use serenity::model::id::ChannelId;
    pub use serenity::model::prelude::{Role, UserId};
    pub use serenity::model::Permissions;
    pub use serenity::prelude::{Context, RwLock};
    pub use serenity::utils::MessageBuilder;
}
