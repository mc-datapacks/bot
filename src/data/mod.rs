use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use serenity::model::prelude::{GuildId, ChannelId};

pub struct VerifyChannel;

impl TypeMapKey for VerifyChannel {
	type Value = HashMap<GuildId, Vec<ChannelId>>;
}