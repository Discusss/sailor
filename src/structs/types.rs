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

    pub fn from_info(string: &String) -> LinkType {
        match string.as_str() {
            "Phishing" => LinkType::Phishing,
            "Malware" => LinkType::Malware,
            "Session Hijacking" => LinkType::SessionHijacking,
            "Cross-site Scripting (XSS)" => LinkType::XSS,
            "Click-Jacking" => LinkType::ClickJacking,
            "Social Engineering" => LinkType::SocialEngineering,
            "IP Grabber" => LinkType::IpGrabber,
            _ => LinkType::Other,
        }
    }

    pub fn from_code(code: &i32) -> LinkType {
        match code {
            0 => LinkType::Phishing,
            1 => LinkType::Malware,
            2 => LinkType::SessionHijacking,
            3 => LinkType::XSS,
            4 => LinkType::ClickJacking,
            5 => LinkType::SocialEngineering,
            6 => LinkType::IpGrabber,
            _ => LinkType::Other,
        }
    }

    pub fn to_code(&self) -> i32 {
        match self {
            LinkType::Phishing => 0,
            LinkType::Malware => 1,
            LinkType::SessionHijacking => 2,
            LinkType::XSS => 3,
            LinkType::ClickJacking => 4,
            LinkType::SocialEngineering => 5,
            LinkType::IpGrabber => 6,
            LinkType::Other => 7,
        }
    }
}