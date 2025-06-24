use crate::agents::base::AgentMemory;

pub fn generate_agent_summary(memory: &AgentMemory) -> String {
    let recon = memory.get("recon_data").unwrap_or(&"N/A".to_string());
    let fuzz = memory.get("fuzz_findings").unwrap_or(&"N/A".to_string());
    let exploit = memory.get("exploit_result.clone()").unwrap_or(&"N/A".to_string());

    format!(
        "[Agent Report Summary]\n{}\n{}\n{}\n",
        recon, fuzz, exploit
    )
}
