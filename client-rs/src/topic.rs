use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Encode, Decode, PartialEq)]
pub struct Topic {
    pub name: String,
}

impl Topic {
    pub fn new(name: String) -> Self {
        Self { name: name }
    }
    pub fn is_name_empty(&self) -> bool {
        self.name.is_empty()
    }
}

impl Default for Topic {
    fn default() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}
