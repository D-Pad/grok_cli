use serde::Deserialize;


#[derive(Deserialize)]
pub struct Grok {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<GrokChoices>,
    pub usage: GrokUsage, 
    pub system_fingerprint: String
}

#[derive(Deserialize)]
pub struct GrokChoices {
    pub index: u16,
    pub message: GrokMessage, 
    pub finish_reason: String
}

#[derive(Deserialize)]
pub struct GrokMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct GrokUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub prompt_tokens_details: PromptTokensDetails 
}

#[derive(Deserialize)]
pub struct PromptTokensDetails {
    pub text_tokens: u32,
    pub audio_tokens: u32,
    pub image_tokens: u32,
    pub cached_tokens: u32
}

