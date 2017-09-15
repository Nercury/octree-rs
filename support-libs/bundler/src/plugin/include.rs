use {StaticId, Result, Bundler};
use util;
use plugin;
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

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

impl plugin::FilesConfig for Config {
    fn type_id(&self) -> &'static str {
        "include"
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

struct Action;

impl Action {
    pub fn new() -> Action {
        Action
    }
}

impl StaticId for Action {
    fn static_id(&self) -> &'static str {
        "include"
    }
}

impl plugin::Files for Action {
    fn deserialize_config(&self, data: &[u8]) -> Result<Box<plugin::FilesConfig>> {
        let mut de = Deserializer::new(data);
        let res: Config = Deserialize::deserialize(&mut de)?;
        Ok(Box::new(res) as Box<plugin::FilesConfig>)
    }
}

pub fn init(bundler: &mut Bundler) {
    bundler.insert_files_plugin(Action::new());
}