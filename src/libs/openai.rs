use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::env;
use dotenv::dotenv;

#[derive(Serialize)]
struct ChatRequest {
  model: String,
  max_tokens: u32,
  messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
  role: String,
  content: String,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
  choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
  message: AssistantMessage,
}

#[derive(Deserialize, Debug)]
pub struct AssistantMessage {
  pub content: String,
}

pub async fn chat_completion(text: &str) -> Result<String, Box<dyn Error>> {
  dotenv().ok();
  let api_key = env::var("OPENAI_API_KEY").unwrap_or("".to_string());

  let client = Client::new();

  let request_body = ChatRequest {
    model: "gpt-4o".to_string(),
    max_tokens: 500,
    messages: vec![
        Message {
          role: "system".to_string(),
          content: "You are a helpful assistant.".to_string(),
        },
        Message {
          role: "user".to_string(),
          content: format!("Summarize the following text:\n\n{}", text),
        },
    ],
  };

  let response = client
    .post("https://api.openai.com/v1/chat/completions")
    .header("Content-Type", "application/json")
    .header("Authorization", format!("Bearer {}", api_key))
    .json(&request_body)
    .send()
    .await?
    .json::<ChatResponse>()
    .await?;

  let choice = response.choices.first().unwrap().message.content.clone();
  Ok(choice)
}