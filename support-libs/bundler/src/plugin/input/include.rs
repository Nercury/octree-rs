use error::{Error, Result};
use Bundler;
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

impl plugin::input::Config for Config {
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

impl plugin::StaticId for Plugin {
    fn static_id(&self) -> &'static str {
        PLUGIN_ID
    }
}

impl plugin::input::Plugin for Plugin {
    fn deserialize_config(&self, data: &[u8]) -> Result<Box<plugin::input::Config>> {
        let mut de = Deserializer::new(data);
        let res: Config = Deserialize::deserialize(&mut de)?;
        Ok(Box::new(res) as Box<plugin::input::Config>)
    }

    fn iter(&self, config: &plugin::Config, crate_path: &Path) -> Result<Box<Iterator<Item=io::Result<Box<plugin::input::File>>>>> {
        let config: &Config = config.any().downcast_ref::<Config>()
            .ok_or_else(|| Error::InvalidConfig { plugin_id: PLUGIN_ID })?;

        let mut from_abs_dir: PathBuf = crate_path.into();
        for item in &config.from_rel_dir {
            from_abs_dir = from_abs_dir.join(item);
        }

        let walkdir = walkdir::WalkDir::new(from_abs_dir);

        Ok(Box::new(
            walkdir.into_iter()
                .filter(|e| e
                    .as_ref()
                    .map(|e| e.file_type().is_file())
                    .unwrap_or(false)
                )
                .map(|item| item
                    .map(|entry| Box::new(entry) as Box<plugin::File>)
                    .map_err(|e| e.into())
                )
        ))
    }
}

impl plugin::File for walkdir::DirEntry {
    fn path(&self) -> &Path {
        self.path()
    }

    fn read(&self) -> Option<Box<plugin::ReadFile>> {
        None
    }

    fn copy(&self) -> Option<Box<plugin::CopyFile>> {
        None
    }

    fn timestamp(&self) -> Option<io::Result<u64>> {
        None
    }
}

pub fn init(bundler: &mut Bundler) {
    bundler.insert_files_plugin(Plugin::new());
}