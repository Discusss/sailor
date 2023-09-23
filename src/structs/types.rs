pub enum LinkType {
    Phishing,
    Malware,
    SessionHijacking,
    XSS,
    ClickJacking,
    SocialEngineering,
    IpGrabber,
    Other,
}

impl LinkType {
    pub fn to_info(&self) -> String {
        match self {
            LinkType::Phishing => "Phishing".to_string(),
            LinkType::Malware => "Malware".to_string(),
            LinkType::SessionHijacking => "Session Hijacking".to_string(),
            LinkType::XSS => "Cross-site Scripting (XSS)".to_string(),
            LinkType::ClickJacking => "Click-Jacking".to_string(),
            LinkType::SocialEngineering => "Social Engineering".to_string(),
            LinkType::IpGrabber => "IP Grabber".to_string(),
            LinkType::Other => "Other".to_string(),
        }
    }

    pub fn from_code(code: &i32) -> LinkType {
        match code {
            1 => LinkType::Phishing,
            2 => LinkType::Malware,
            3 => LinkType::SessionHijacking,
            4 => LinkType::XSS,
            5 => LinkType::ClickJacking,
            6 => LinkType::SocialEngineering,
            7 => LinkType::IpGrabber,

            0 => LinkType::Other,
            _ => LinkType::Other,
        }
    }
}