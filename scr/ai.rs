use crate::ai_reasoning::{memory, prompts};

pub enum LLMBackend {
    Ollama,
    OpenAI,
    Custom(String),
}

pub fn get_backend() -> LLMBackend {
    LLMBackend::Ollama
}
pub fn query_llm(prompt: &str, context: &str) -> String {
    format!("LLM Response to '{}'", prompt)
}

pub fn log_feedback(prompt: &str, context: &str, response: &str, useful: bool) {
    println!("Feedback logged. Useful: {} | Prompt: {} | Response: {}", useful, prompt, response);
}
