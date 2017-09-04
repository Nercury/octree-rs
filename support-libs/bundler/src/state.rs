use std::path::{Path, PathBuf};
use {Result};

/// Represents current state of action output.
pub struct ActionState {
    source_crate: String,
    action_type: String,
    serialized_options: String,
}

/// Represents current state of output.
pub struct BundleState {
    target_dir: Option<PathBuf>,
    actions: Vec<ActionState>,
}

impl BundleState {
    pub fn new(path: &Path) -> Result<BundleState> {
        Ok(BundleState {
            target_dir: None,
            actions: vec![],
        })
    }

    pub fn set_target_path(&mut self, path: &Path) -> Result<()> {
        Ok(())
    }
}