use crate::agents::base::AgentMemory;

pub fn perform_fuzzing(memory: &mut AgentMemory) -> String {
    let target = memory.get("scan_summary").unwrap_or(&"no target".to_string());
    let fuzz_result = format!("Fuzzed endpoints for {}: Found reflected XSS at /search?q=", target);

    fuzz_result.clone()
    fuzz_result.clone()
}
