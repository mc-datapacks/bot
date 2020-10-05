use super::data::Error;
use super::prelude::*;
use super::{Database, VerifyChannel};
use random_color::RandomColor;
use serenity::builder::EditRole;
use serenity::framework::standard::macros::check;
use serenity::framework::standard::CheckResult;
use std::sync::Arc;

#[check]
#[name = "is_in_verify_channel"]
pub fn is_in_verify_channel(context: &mut Context, message: &Message) -> CheckResult {
    let data = context.data.read();
    if let Some(database) = data.get::<VerifyChannel>() {
        if database.exists(message.channel_id) {
            return CheckResult::Success;
        }
    }

    CheckResult::new_user("You're not in a verification channel")
}

pub fn get_database<F, T, E>(context: &mut Context, f: F) -> Result<T, E>
where
    F: FnOnce(&mut Database) -> Result<T, E>,
    E: From<Error>,
{
    let mut data = context.data.write();

    match data.get_mut::<VerifyChannel>() {
        Some(database) => f(database),
        None => Err(Error::MissingDatabase.into()),
    }
}

pub type SafeGuild = Arc<RwLock<Guild>>;

pub fn get_guild<F, T, E>(context: &Context, message: &Message, f: F) -> Result<T, E>
where
    F: FnOnce(SafeGuild) -> Result<T, E>,
    E: From<Error>,
{
    match message.guild(&context) {
        Some(guild) => f(guild),
        None => Err(Error::OutsideGuild.into()),
    }
}

pub fn invoke_command(message: &Message) {
    log::info!(
        "{user} invoke `{command}`",
        user = message.author.tag(),
        command = message.content
    );
}

pub fn role_creator<'a>(role: &'a mut EditRole, name: &str) -> &'a mut EditRole {
    let [r, g, b] = RandomColor::new().to_rgb_array();
    let color = convert_rgb(r, g, b);

    role.name(name)
        .permissions(Permissions::empty())
        .colour(color)
}

pub fn convert_rgb(r: u32, g: u32, b: u32) -> u64 {
    (r as u64) << 16 | (g as u64) << 8 | b as u64
}

pub fn role_by_name(name: &str, context: &Context, message: &Message) -> Result<Role, Error> {
    let guild = message
        .guild(&context.cache)
        .ok_or_else(|| Error::OutsideGuild)?;
    let guild = guild.read();
    guild
        .role_by_name(name)
        .cloned()
        .ok_or_else(|| Error::MissingRole)
}

#[macro_export]
macro_rules! report {
    ($x:expr =>> $context:expr => $message:expr) => {
        match $x {
            Some(v) => v,
            None => return Ok(()),
        }
    };
    ($x:expr => $context:expr => $message:expr) => {
        match $x {
            Ok(v) => v,
            Err(e) => {
                bail!(e => $context => $message);
            }
        }
    };
}

#[macro_export]
macro_rules! bail {
    ($x:expr => $context:expr => $message:expr) => {
        let msg = format!("{}", $x);
        $message.channel_id.say(&$context.http, msg)?;
        bail!($x);
    };
    ($message:expr) => {
        return Err($message.into());
    };
}
