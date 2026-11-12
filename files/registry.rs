//! Windows registry read helpers for Upnote Sync.

pub const APP_KEY: &str = "Software\\Upnote\\Upnote Sync";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Upnote Sync";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Upnote Sync") },
        RegValue { name: "Version",     value: String::from("v4.0.26") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Upnote Sync") },
        RegValue { name: "Publisher",       value: String::from("Upnote") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Upnote Sync\\uninstall.exe\"") },
    ]
}
