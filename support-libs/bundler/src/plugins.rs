use {Error, Result};
use std::collections::HashMap;

pub struct Plugins<T> {
    plugin_type: &'static str,
    plugins: HashMap<&'static str, T>,
}

impl<T> Plugins<T> {
    pub fn new(plugin_type: &'static str) -> Plugins<T> {
        Plugins {
            plugin_type,
            plugins: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: &'static str, action: T) {
        self.plugins.insert(id, action);
    }

    pub fn get(&self, id: &str) -> Result<&T> {
        Ok(
            self.plugins
                .get(id)
                .ok_or_else(|| Error::PluginNotInitialized {
                    plugin_type: self.plugin_type,
                    plugin_id: id.to_string()
                })?
        )
    }
}

pub trait StaticId {
    fn static_id(&self) -> &'static str;
}