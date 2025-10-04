use serde::{Deserialize, Serialize};

use crate::topic::Topic;
pub struct Message {
    group_id: String,
    topic: Topic,
    data: String,
}

impl Message {
    pub fn is_group_id_empty(&self) -> bool {
        self.group_id.is_empty()
    }
    pub fn is_topic_id_empty(&self) -> bool {
        self.topic.is_name_empty()
    }
}
