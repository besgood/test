
pub mod auth;
pub mod burp_parser;
pub mod proxy_chain;
pub mod replay;
pub mod session_store;

pub fn analyze_burp_log(file: &str, _opt: Option<&str>) {
    println!("Analyzing Burp file: {}", file);
}
