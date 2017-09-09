use {ActionType, ActionConfig};
use {Result, Error};
use util;
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use std::any::Any;

pub struct Copy;

impl Copy {
    pub fn new() -> Copy {
        Copy
    }
}

impl ActionType for Copy {
    fn id(&self) -> &'static str {
        "copy"
    }

    fn boxed(self) -> Box<ActionType> {
        Box::new(self) as Box<ActionType>
    }

    fn deserialize_config(&self, data: &[u8]) -> Result<Box<ActionConfig>> {
        let mut de = Deserializer::new(data);
        let res: CopyConfig = Deserialize::deserialize(&mut de)?;
        Ok(Box::new(res) as Box<ActionConfig>)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyConfig {
    from_rel_dir: Vec<String>,
    to_rel_dir: Vec<String>,
    action_hash: Vec<u8>,
}

impl CopyConfig {
    pub fn new(from_rel_dir: &[&str], to_rel_dir: &[&str]) -> CopyConfig {
        let mut hasher = util::hash::new();
        util::hash::write_slice_of_str(&mut hasher, from_rel_dir);
        util::hash::write_slice_of_str(&mut hasher, to_rel_dir);
        let hash = hasher.finish();

        CopyConfig {
            from_rel_dir: from_rel_dir.iter().map(|s| s.to_string()).collect(),
            to_rel_dir: to_rel_dir.iter().map(|s| s.to_string()).collect(),
            action_hash: hash,
        }
    }
}

impl ActionConfig for CopyConfig {
    fn type_id(&self) -> &'static str {
        "copy"
    }

    fn boxed(self) -> Box<ActionConfig> {
        Box::new(self) as Box<ActionConfig>
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