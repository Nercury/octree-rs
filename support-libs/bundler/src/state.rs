use std::path::{Path, PathBuf};
use std::collections::HashMap;
use action_types::ActionTypes;
use Result;

/// Represents current state of action output.
pub struct ActionState {
    source_crate: String,
    action_type: String,
    serialized_options: String,
}

pub struct OutputConfig {
    pub target_dir: Option<PathBuf>,
}

/// Represents current state of output.
pub struct BundleState {
    output_config: Option<OutputConfig>,
    actions: Vec<ActionState>,
}

impl BundleState {
    pub fn new(path: &Path) -> Result<BundleState> {
        Ok(BundleState {
            output_config: None,
            actions: vec![],
        })
    }

    pub fn configure_output(
        &mut self,
        output_config: OutputConfig
    ) -> Result<()> {
        self.output_config = Some(output_config);
        Ok(())
    }
}