use crate::database::Database;
use crate::trading::RequestDatabase;
use serenity::prelude::TypeMapKey;

mod error;
pub use error::*;

pub struct VerifyChannel;

impl TypeMapKey for VerifyChannel {
    type Value = Database;
}

pub struct RoleRequest;

impl TypeMapKey for RoleRequest {
    type Value = RequestDatabase;
}
