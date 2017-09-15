use {Result};

pub mod include;

pub trait Files {
    fn deserialize_config(&self, data: &[u8]) -> Result<Box<FilesConfig>>;
}

pub trait FilesConfig {
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