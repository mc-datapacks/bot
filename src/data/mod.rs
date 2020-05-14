use serenity::prelude::TypeMapKey;
use crate::database::Database;

mod error;
pub use error::*;

pub struct VerifyChannel;

impl TypeMapKey for VerifyChannel {
	type Value = Database;
}
