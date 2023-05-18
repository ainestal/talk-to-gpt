use std::fmt;

use serde::{Deserialize, Serialize};

use crate::gpt::Message;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatContext {
    pub model: String,
    pub messages: Vec<Message>,
}

impl ChatContext {
    pub fn new() -> ChatContext {
        ChatContext {
            model: "gpt-4".to_string(),
            messages: Vec::new(),
        }
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }
}

impl fmt::Display for ChatContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ \"model\": {}, \"messages\": [", self.model)?;
        for message in self.messages.iter() {
            write!(f, "{}, ", message)?;
        }
        write!(f, "] }}")
    }
}
