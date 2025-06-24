pub trait ScanTool {
    fn name(&self) -> &str;
    fn run(&self);
}

pub struct FFUFScan {
    pub url: String,
    pub wordlist: String,
}
impl ScanTool for FFUFScan {
    fn name(&self) -> &str { "FFUF" }
    fn run(&self) {}
}

pub struct AmassScan {
    pub domain: String,
}
impl ScanTool for AmassScan {
    fn name(&self) -> &str { "Amass" }
    fn run(&self) {}
}

pub struct HttpxScan {
    pub urls: Vec<String>,
}
impl ScanTool for HttpxScan {
    fn name(&self) -> &str { "Httpx" }
    fn run(&self) {}
}

pub struct NucleiScan {
    pub urls: Vec<String>,
}
impl ScanTool for NucleiScan {
    fn name(&self) -> &str { "Nuclei" }
    fn run(&self) {}
}