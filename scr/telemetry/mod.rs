
pub mod logger;

use crate::ai::LLMBackend;
use crate::agents::base::AgentMemory;
use crate::react::ReasoningStep;
use crate::config::CONFIG;

pub fn log_event(event: &str) {
    println!("LOG: {}", event);
}


    println!("EVENT: {}", event);
}
