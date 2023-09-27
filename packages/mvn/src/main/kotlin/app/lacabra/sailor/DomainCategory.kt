package app.lacabra.sailor

/**
 * The category of the domain
 * (https://github.com/Discusss/sailor/blob/main/src/structs/types.rs)
 */
enum class DomainCategory {
    Other,
    Phishing,
    Malware,
    `Session Hijacking`,
    `Cross-site Scripting (XSS)`,
    `Click-Jacking`,
    `Social Engineering`,
    `IP Grabber`,
    Scam,
}