//! Blocking OpenRouter chat client.

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use serde_json::json;

use crate::model::{ChatClient, ChatMessage, Role};

const DEFAULT_BASE: &str = "https://openrouter.ai/api/v1";

pub struct OpenRouterClient {
    http: reqwest::blocking::Client,
    api_key: String,
    base_url: String,
}

impl OpenRouterClient {
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("OPENROUTER_API_KEY")
            .context("OPENROUTER_API_KEY is required for enrich-comments")?;
        if api_key.trim().is_empty() {
            bail!("OPENROUTER_API_KEY is empty");
        }
        let base_url = std::env::var("OPENROUTER_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE.into());
        Self::new(api_key, base_url)
    }

    pub fn new(api_key: impl Into<String>, base_url: impl Into<String>) -> Result<Self> {
        let http = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .context("building HTTP client")?;
        Ok(Self {
            http,
            api_key: api_key.into(),
            base_url: base_url.into().trim_end_matches('/').to_owned(),
        })
    }
}

impl ChatClient for OpenRouterClient {
    fn chat(&self, model: &str, messages: &[ChatMessage], json_mode: bool) -> Result<String> {
        let msgs: Vec<_> = messages
            .iter()
            .map(|m| {
                json!({
                    "role": match m.role {
                        Role::System => "system",
                        Role::User => "user",
                        Role::Assistant => "assistant",
                    },
                    "content": m.content,
                })
            })
            .collect();

        let mut body = json!({
            "model": model,
            "messages": msgs,
        });
        if json_mode {
            body["response_format"] = json!({ "type": "json_object" });
        }

        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/robby/robby")
            .header("X-Title", "robby")
            .json(&body)
            .send()
            .with_context(|| format!("POST {url}"))?;

        let status = response.status();
        let text = response.text().context("reading OpenRouter body")?;
        if !status.is_success() {
            bail!("OpenRouter {status}: {text}");
        }

        let parsed: ChatCompletion = serde_json::from_str(&text)
            .with_context(|| format!("decoding OpenRouter JSON: {text}"))?;
        parsed
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .context("OpenRouter response missing choices[0].message.content")
    }
}

#[derive(Debug, Deserialize)]
struct ChatCompletion {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChoiceMessage {
    content: Option<String>,
}
