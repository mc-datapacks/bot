use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot find database")]
    MissingDatabase,

    #[error("You are not running this from inside a guild")]
    OutsideGuild,

    #[error("Role's name cannot be empty")]
    EmptyRoleName,

    #[error(transparent)]
    Serenity(#[from] serenity::Error),

    #[error("Unknown channel")]
    UnknownChannel,
}
