use {ActionType, ActionConfig};

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
}

pub struct CopyConfig {
    from_rel_dir: Vec<String>,
    to_rel_dir: Vec<String>,
}

impl CopyConfig {
    pub fn new(from_rel_dir: &[&str], to_rel_dir: &[&str]) -> CopyConfig {
        CopyConfig {
            from_rel_dir: from_rel_dir.iter().map(|s| s.to_string()).collect(),
            to_rel_dir: to_rel_dir.iter().map(|s| s.to_string()).collect(),
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
}