use crate::database::Database;
use serenity::prelude::TypeMapKey;

mod error;
pub use error::*;

pub struct VerifyChannel;

impl TypeMapKey for VerifyChannel {
    type Value = Database;
}
