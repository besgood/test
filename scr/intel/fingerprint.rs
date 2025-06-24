//! Entry point for enhanced recon and decision logic

pub mod fingerprint;
pub mod recon;
pub mod decision;

use crate::intel::recon::passive_recon;
use crate::intel::decision::suggest_tools;

pub fn analyze_target(target: &str) {
    println!("🔍 Passive recon for: {}", target);
    let recon = passive_recon(target);

    println!("🔬 Fingerprinting stack: {}", target);
    let techs = fingerprint_target(target);

    println!("🤖 AI Tool Suggestions:");
    let suggestions = suggest_tools(&recon, &techs);

    for tool in suggestions {
        println!(" - {}", tool);
    }
}

pub fn fingerprint_target(target: &str) -> String {
    format!("Fingerprinting {}", target)
}
