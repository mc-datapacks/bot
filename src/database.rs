use super::Error;
use anyhow::Result;
use log::{debug, info};
use serenity::model::id::ChannelId;
use std::fs;
use std::fs::File;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub struct Database {
    // Note: Serenity currently doesn't support async runtime so we have to use this for now
    path: PathBuf,
    cache: HashSet<ChannelId>,
}

fn prepare_database(path: &Path) -> Result<()> {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, "[]")?;
    }

    Ok(())
}

impl Database {
    fn prepare(&self) -> Result<()> {
        prepare_database(&self.path)
    }

    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        info!("Initialize new database");
        let path = path.into();

        prepare_database(&path)?;

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

    pub fn remove_channel(&mut self, id: ChannelId) -> Result<()> {
        debug!("Removed {} from database", id);
        let success = self.cache.remove(&id);
        self.update_database()?;

        if success {
            Ok(())
        } else {
            Err(Error::UnknownChannel.into())
        }
    }

    pub fn update_database(&self) -> Result<()> {
        debug!("Update database");

        self.prepare()?;
        let writer = File::create(&self.path)?;
        serde_json::to_writer(writer, &self.cache)?;
        Ok(())
    }

    pub fn exists(&self, id: ChannelId) -> bool {
        self.cache.contains(&id)
    }

    pub fn intersect<'a>(&'a self, list: &'a HashSet<ChannelId>) -> HashSet<&'a ChannelId> {
        self.cache.intersection(list).collect()
    }

    pub fn remove_channels(&mut self, channels: HashSet<ChannelId>) -> Result<()> {
        debug!("Removed {:?} from database", channels);
        channels.iter().for_each(|id| {
            self.cache.remove(id);
        });
        self.update_database()
    }
}
