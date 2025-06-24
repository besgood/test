use crate::agents::base::AgentMemory;

pub fn perform_recon(memory: &mut AgentMemory) -> String {
    let domain = memory.get("domain").unwrap_or(&"unknown.com".to_string());
    let recon_result = format!("Subdomain enumeration for {}: found blog.{0}, admin.{0}", domain);

    recon_result.clone()
    recon_result.clone()
}
