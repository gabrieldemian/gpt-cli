use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub text: Option<String>,
    pub index: u32,
    pub logprobs: Option<String>,
    pub finish_reason: Option<String>,
    pub message: Option<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionBody {
    pub model: String,
    pub prompt: String,
    pub max_tokens: i32,
    pub temperature: f64,
    pub stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResp {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: u64,
    pub model: Option<String>,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}
