//! Static build info for Upnote Sync.

pub const BUILD_VERSION: &str  = "v4.0.26";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "a35092aa7be2";
pub const BUILD_TIMESTAMP: u64 = 1716005430;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Upnote Sync", BUILD_VERSION, BUILD_CHANNEL)
}
