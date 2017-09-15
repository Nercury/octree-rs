use {Error, Result};
use std::collections::HashMap;

pub struct Plugins<T> {
    types: HashMap<&'static str, T>,
}

impl<T> Plugins<T> {
    pub fn new() -> Plugins<T> {
        Plugins {
            types: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: &'static str, action: T) {
        self.types.insert(id, action);
    }

    pub fn get(&self, id: &str) -> Result<&T> {
        Ok(self.types
            .get(id)
            .ok_or_else(|| Error::FilesActionNotInitialized { plugin_id: id.to_string() })?)
    }
}

pub trait StaticId {
    fn static_id(&self) -> &'static str;
}