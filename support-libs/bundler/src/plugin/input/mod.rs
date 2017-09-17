use error::{Result};
use std::io;
use std::path::{Path};
use plugin::File;
use plugin::Config;

pub mod include;

pub trait Plugin {
    fn deserialize_config(&self, data: &[u8]) -> Result<Box<Config>>;
    fn iter(&self, config: &Config, crate_path: &Path) -> Result<Box<Iterator<Item=io::Result<Box<File>>>>>;
}