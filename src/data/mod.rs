use serenity::model::prelude::{ChannelId, GuildId};
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;

pub struct VerifyChannel;

impl TypeMapKey for VerifyChannel {
	type Value = HashMap<GuildId, Vec<ChannelId>>;
}
