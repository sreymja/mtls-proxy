use anyhow::Result;
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub n: Option<usize>,
    pub stop: Option<Vec<String>>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub logit_bias: Option<HashMap<String, f32>>,
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Serialize)]
pub struct Choice {
    pub index: usize,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Serialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChoiceDelta>,
}

#[derive(Debug, Serialize)]
pub struct ChoiceDelta {
    pub index: usize,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Delta {
    pub role: Option<String>,
    pub content: Option<String>,
}

pub struct ChatResponseGenerator {
    responses: Vec<String>,
}

impl ChatResponseGenerator {
    pub fn new() -> Self {
        let responses = vec![
            "I understand your question. Let me provide a helpful response based on the information available.",
            "That's an interesting point. Here's what I can tell you about that topic.",
            "Based on my knowledge, I can help you with that. Here's what I found.",
            "I'd be happy to assist you with that. Let me break it down for you.",
            "That's a great question. Here's my perspective on the matter.",
            "I can help you with that. Here's what you need to know.",
            "Let me provide you with some information about that topic.",
            "I understand what you're asking. Here's a detailed response.",
            "That's an excellent question. Let me explain this for you.",
            "I can certainly help with that. Here's what I recommend.",
            "Based on the context, here's what I think would be most helpful.",
            "I'd be glad to assist you. Here's my analysis of the situation.",
            "That's a valid concern. Let me address that for you.",
            "I can provide some insights on that topic. Here's what I know.",
            "Let me help you understand this better. Here's my explanation.",
        ];

        Self { responses: responses.into_iter().map(String::from).collect() }
    }

    pub fn generate_response(&self, request: &ChatCompletionRequest) -> Result<ChatCompletionResponse> {
        let _rng = rand::thread_rng();
        
        // Generate a realistic response based on the user's message
        let user_message = request.messages.last()
            .map(|m| m.content.as_str())
            .unwrap_or("Hello");
        
        let response_content = self.generate_content(user_message);
        
        // Calculate token counts (rough estimation)
        let prompt_text = request.messages.iter()
            .map(|m| m.content.as_str())
            .collect::<Vec<&str>>()
            .join(" ");
        let prompt_tokens = self.estimate_tokens(&prompt_text);
        let completion_tokens = self.estimate_tokens(&response_content);
        
        Ok(ChatCompletionResponse {
            id: format!("chatcmpl-{}", Uuid::new_v4().to_string().replace("-", "")),
            object: "chat.completion".to_string(),
            created: Utc::now().timestamp(),
            model: request.model.clone(),
            choices: vec![Choice {
                index: 0,
                message: Message {
                    role: "assistant".to_string(),
                    content: response_content,
                    name: None,
                },
                finish_reason: "stop".to_string(),
            }],
            usage: Usage {
                prompt_tokens,
                completion_tokens,
                total_tokens: prompt_tokens + completion_tokens,
            },
        })
    }

    pub fn generate_streaming_response(&self, request: &ChatCompletionRequest) -> Result<Vec<ChatCompletionChunk>> {
        let response_content = self.generate_content(
            request.messages.last()
                .map(|m| m.content.as_str())
                .unwrap_or("Hello")
        );
        
        let chunks = self.chunk_response(&response_content);
        let mut result = Vec::new();
        
        for (index, chunk) in chunks.iter().enumerate() {
            let is_last = index == chunks.len() - 1;
            
            result.push(ChatCompletionChunk {
                id: format!("chatcmpl-{}", Uuid::new_v4().to_string().replace("-", "")),
                object: "chat.completion.chunk".to_string(),
                created: Utc::now().timestamp(),
                model: request.model.clone(),
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: Delta {
                        role: if index == 0 { Some("assistant".to_string()) } else { None },
                        content: Some(chunk.clone()),
                    },
                    finish_reason: if is_last { Some("stop".to_string()) } else { None },
                }],
            });
        }
        
        Ok(result)
    }

    fn generate_content(&self, user_message: &str) -> String {
        let mut rng = rand::thread_rng();
        
        // Select a base response
        let base_response = self.responses[rng.gen_range(0..self.responses.len())].clone();

        // Customize based on user message
        let content = if user_message.to_lowercase().contains("hello") || user_message.to_lowercase().contains("hi") {
            "Hello! I'm here to help you. How can I assist you today?"
        } else if user_message.to_lowercase().contains("help") {
            "I'd be happy to help you! What specific question or task do you need assistance with?"
        } else if user_message.to_lowercase().contains("explain") {
            "I'd be glad to explain that for you. Could you provide a bit more context so I can give you the most helpful explanation?"
        } else if user_message.to_lowercase().contains("code") || user_message.to_lowercase().contains("programming") {
            "I can help you with programming questions. What language or framework are you working with?"
        } else if user_message.to_lowercase().contains("thank") {
            "You're very welcome! I'm glad I could help. Is there anything else you'd like to know?"
        } else {
            &*base_response
        };
        
        // Add some variation to make responses more realistic
        let variations = vec![
            content.to_string(),
            format!("{} I hope this information is helpful to you.", content),
            format!("{} Please let me know if you need any clarification.", content),
            format!("{} Feel free to ask if you have any follow-up questions.", content),
        ];
        
        variations[rng.gen_range(0..variations.len())].clone()
    }

    fn chunk_response(&self, content: &str) -> Vec<String> {
        // Split content into chunks for streaming
        let words: Vec<&str> = content.split_whitespace().collect();
        let chunk_size = (words.len() / 5).max(1); // 5 chunks minimum
        
        let mut chunks = Vec::new();
        for chunk in words.chunks(chunk_size) {
            chunks.push(chunk.join(" "));
        }
        
        chunks
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        // Rough estimation: 1 token ≈ 4 characters
        (text.len() + 3) / 4
    }
}

impl Default for ChatResponseGenerator {
    fn default() -> Self {
        Self::new()
    }
}
