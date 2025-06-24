pub mod c2 {
    pub struct C2Session;
}

pub mod post_exploitation {
    pub fn suggest_post_exploitation() {}
}

pub mod sliver {
    pub fn launch_sliver_listener() {}
}

pub mod mythic {
    pub fn start_mythic_services() {}
}

pub mod cobalt {
    pub fn launch_cobalt_strike_teamserver() {}
}

impl C2Session {
    pub fn new(_id: &str, _ip: &str, _user: &str, _role: &str) -> Self {
        C2Session {}
    }
}

pub fn launch_cobalt_strike_teamserver(_ip: &str, _pw: &str) {}

pub fn suggest_post_exploitation() -> Vec<String> {
    vec!["dump creds".into(), "exfil files".into()]
}


impl C2Session {
    pub fn new(id: &str, ip: &str, user: &str, role: &str) -> Self {
        C2Session
    }
}


    vec!["Dump creds".to_string(), "Escalate privileges".to_string()]
}
