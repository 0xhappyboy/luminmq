use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Topic {
    name: String,
}

impl Topic {
    pub fn is_name_empty(&self) -> bool {
        self.name.is_empty()
    }
}
