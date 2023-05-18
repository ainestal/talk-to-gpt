use std::fmt;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::chat_context::ChatContext;

const URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ \"role\": {}, \"content\": {} }}",
            self.role, self.content
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    index: u64,
    message: Message,
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    id: String,
    object: String,
    created: u64,
    pub choices: Vec<Choice>,
    usage: Usage,
}

pub struct GPT {
    client: reqwest::Client,
    pub context: ChatContext,
    openai_api_token: String,
}

impl GPT {
    pub fn new(openai_api_token: String) -> Result<GPT> {
        let client = reqwest::Client::new();
        let context = ChatContext::new();
        Ok(GPT {
            client,
            context,
            openai_api_token,
        })
    }

    pub async fn completion(&mut self, input: String) -> Result<Message> {
        self.context.push(Message {
            role: "user".to_string(),
            content: input,
        });

        let response = self
            .client
            .post(URL)
            .bearer_auth(&self.openai_api_token)
            .header("Content-Type", "application/json")
            .json(&self.context)
            .send()
            .await
            .context(format!("Failed to receive the response from {}", URL))?
            .text()
            .await
            .context("Failed to retrieve the content of the response")?;

        let answer: ChatResponse = serde_json::from_str(&response)?;
        Ok(answer.choices[0].message.clone())
    }
}
