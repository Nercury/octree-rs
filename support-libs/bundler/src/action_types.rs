use std::collections::HashMap;
use {Error, Result};

pub struct ActionTypes {
    types: HashMap<&'static str, Box<ActionType>>,
}

impl ActionTypes {
    pub fn new() -> ActionTypes {
        ActionTypes {
            types: HashMap::new(),
        }
    }

    pub fn insert<T: ActionType + 'static>(&mut self, action: T) {
        self.types.insert(action.id(), action.boxed());
    }

    pub fn get(&self, id: &str) -> Result<&ActionType> {
        Ok(&**self.types
            .get(id)
            .ok_or_else(|| Error::ActionTypeNotInitialized { type_id: id.to_string() })?)
    }
}

pub trait ActionType {
    fn id(&self) -> &'static str;
    fn boxed(self) -> Box<ActionType>;
}

pub trait ActionConfig {
    /// Id of action type that can use this configuration.
    fn type_id(&self) -> &'static str;

    /// Create trait object.
    fn boxed(self) -> Box<ActionConfig>;

    /// Byte sequence that uniquely identifies this action.
    ///
    /// Different hash means different action. If some configuration does not differentiate the
    /// action, leave it out of the hash.
    ///
    /// As an example, a different file path might mean different action, while the compression
    /// algorithm option might indicate the same action, but with different parameters.
    fn config_hash(&self) -> &[u8];
}

