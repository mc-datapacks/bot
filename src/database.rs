use anyhow::Result;
use serenity::model::id::{ChannelId};
use std::{collections::HashSet, path::PathBuf};
use log::{debug, info};
use std::fs::File;
use std::fs;

pub struct Database {
	// Note: Serenity currently doesn't support async runtime so we have to use this for now
	path: PathBuf,
	cache: HashSet<ChannelId>
}

impl Database {
	fn prepare(&self) -> Result<()> {
		if let Some(parent) = self.path.parent() {
			fs::create_dir_all(parent)?;
		}

		fs::write(&self.path, "[]")?;
		Ok(())
	}

	pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
		info!("Initialize new database");
		let path = path.into();
		let reader = File::open(&path)?;
		let cache = serde_json::from_reader(reader).unwrap_or_default();
		let result = Self { path, cache };
		Ok(result)
	}

	pub fn add_channel(&mut self, id: ChannelId) -> Result<()> {
		debug!("Added {} to database", id);
		self.cache.insert(id);
		self.update_database()
	}

	pub fn remove_channel(&mut self, id: &ChannelId) -> Result<()> {
		debug!("Removed {} from database", id);
		self.cache.remove(id);
		self.update_database()
	}

	pub fn update_database(&self) -> Result<()> {
		debug!("Update database");

		if !self.path.exists() {
			self.prepare()?;
		}

		let writer = File::open(&self.path)?;
		serde_json::to_writer(writer, &self.cache)?;
		Ok(())
	}

	pub fn exists(&self, id: &ChannelId) -> bool {
		self.cache.contains(id)
	}

	pub fn intersect<'a>(&'a self, list: &'a HashSet<ChannelId>) -> HashSet<&'a ChannelId> {
		self.cache.intersection(list).collect()
	}

	pub fn remove_channels(&mut self, channels: HashSet<ChannelId>) -> Result<()> {
		debug!("Removed {:?} from database", channels);
		channels.iter()
			.for_each(|id| { self.cache.remove(id); });
		self.update_database()
	}
}
