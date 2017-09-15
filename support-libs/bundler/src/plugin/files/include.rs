use {StaticId, Result, Error, Bundler};
use util;
use plugin;
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use std::any::Any;
use std::path::{Path, PathBuf};
use std::io;
use walkdir;

const PLUGIN_ID: &str = "include";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    from_rel_dir: Vec<String>,
    to_rel_dir: Vec<String>,
    action_hash: Vec<u8>,
}

impl Config {
    pub fn new(from_rel_dir: &[&str], to_rel_dir: &[&str]) -> Config {
        let mut hasher = util::hash::new();
        util::hash::write_slice_of_str(&mut hasher, from_rel_dir);
        util::hash::write_slice_of_str(&mut hasher, to_rel_dir);
        let hash = hasher.finish();

        Config {
            from_rel_dir: from_rel_dir.iter().map(|s| s.to_string()).collect(),
            to_rel_dir: to_rel_dir.iter().map(|s| s.to_string()).collect(),
            action_hash: hash,
        }
    }
}

impl plugin::files::Config for Config {
    fn any(&self) -> &Any {
        self as &Any
    }

    fn type_id(&self) -> &'static str {
        PLUGIN_ID
    }

    fn config_hash(&self) -> &[u8] {
        &self.action_hash[..]
    }

    fn serialize(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        Serialize::serialize(self, &mut Serializer::new(&mut buf)).unwrap();
        return Ok(buf);
    }
}

struct Plugin;

impl Plugin {
    pub fn new() -> Plugin {
        Plugin
    }
}

impl StaticId for Plugin {
    fn static_id(&self) -> &'static str {
        PLUGIN_ID
    }
}

impl plugin::files::Plugin for Plugin {
    fn deserialize_config(&self, data: &[u8]) -> Result<Box<plugin::files::Config>> {
        let mut de = Deserializer::new(data);
        let res: Config = Deserialize::deserialize(&mut de)?;
        Ok(Box::new(res) as Box<plugin::files::Config>)
    }

    fn iter(&self, crate_path: &Path, config: &plugin::files::Config) -> Result<Box<Iterator<Item=io::Result<plugin::files::File>>>> {
        let config: &Config = config.any().downcast_ref::<Config>()
            .ok_or_else(|| Error::InvalidConfig { plugin_id: PLUGIN_ID })?;

        let mut from_abs_dir: PathBuf = crate_path.into();
        for item in &config.from_rel_dir {
            from_abs_dir = from_abs_dir.join(item);
        }

        Ok(Box::new(Iter {
            from_abs_dir: from_abs_dir.clone(),
            inner: walkdir::WalkDir::new(from_abs_dir).into_iter(),
        }))
    }
}

pub fn init(bundler: &mut Bundler) {
    bundler.insert_files_plugin(Plugin::new());
}

struct Iter {
    from_abs_dir: PathBuf,
    inner: walkdir::Iter,
}

impl Iterator for Iter {
    type Item = io::Result<plugin::files::File>;

    fn next(&mut self) -> Option<io::Result<plugin::files::File>> {
        match self.inner.next() {
            Some(Ok(entry)) => Some(Ok(plugin::files::File {
                read: None,
                timestamp: None,
                copy: None,
                path: entry.path().into(),
            })),
            None => None,
            Some(Err(e)) => Some(Err(e.into())),
        }
    }
}