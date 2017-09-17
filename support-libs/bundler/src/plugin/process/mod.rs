use error::Result;
use plugin::{Config, File};
use std::io;

pub trait Plugin {
    fn deserialize_config(&self, data: &[u8]) -> Result<Box<Config>>;
    fn iter(&self, config: &Config, input: &Iterator<Item=io::Result<Box<File>>>)
        -> Result<Box<Iterator<Item=io::Result<Box<File>>>>>;
}