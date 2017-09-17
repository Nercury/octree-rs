use error::{Error, Result};
use std::collections::HashMap;
use std::any::Any;
use std::io;
use std::fs;
use std::path::{Path};
use plugin;

pub mod input;
pub mod process;

pub struct Set<T> {
    plugin_type: &'static str,
    plugins: HashMap<&'static str, T>,
}

impl<T> Set<T> {
    pub fn new(plugin_type: &'static str) -> Set<T> {
        Set {
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

pub trait Config: Any {
    fn any(&self) -> &Any;

    /// Id of action type that can use this configuration.
    fn type_id(&self) -> &'static str;

    /// Byte sequence that uniquely identifies this action.
    ///
    /// Different hash means different action. If some configuration does not differentiate the
    /// action, leave it out of the hash.
    ///
    /// As an example, a different file path might mean different action, while the compression
    /// algorithm option might indicate the same action, but with different parameters.
    fn config_hash(&self) -> &[u8];

    fn serialize(&self) -> Result<Vec<u8>>;
}

pub trait File {
    /// Path to virtual file.
    fn path(&self) -> &Path;
    /// File reader object.
    fn read(&self) -> Option<Box<plugin::ReadFile>>;
    /// Optional file copy action.
    fn copy(&self) -> Option<Box<plugin::CopyFile>>;
    /// Optional file timestamp.
    fn timestamp(&self) -> Option<io::Result<u64>>;
}

pub trait ReadFile {
    fn read_file(&self) -> io::Result<fs::File>;
}

pub trait CopyFile {
    fn copy_file(&self) -> io::Result<()>;
}