use {Result};
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::any::Any;

pub mod include;

pub struct File {
    /// Path to virtual file.
    pub path: PathBuf,
    /// File reader object.
    pub read: Option<Box<ReadFile>>,
    /// Optional file timestamp action.
    pub timestamp: Option<Box<FileTimestamp>>,
    /// Optional file copy action.
    pub copy: Option<Box<CopyFile>>
}

pub trait FileTimestamp {
    fn file_timestamp(&self) -> io::Result<u64>;
}

pub trait ReadFile {
    fn read_file(&self, target_path: &Path) -> io::Result<fs::File>;
}

pub trait CopyFile {
    fn copy_file(&self, target_path: &Path) -> io::Result<()>;
}

pub trait Plugin {
    fn deserialize_config(&self, data: &[u8]) -> Result<Box<Config>>;
    fn iter(&self, crate_path: &Path, config: &Config) -> Result<Box<Iterator<Item=io::Result<File>>>>;
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