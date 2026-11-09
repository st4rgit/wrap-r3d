//! Windows registry read helpers for Wrap R3D.

pub const APP_KEY: &str = "Software\\Wrap\\Wrap R3D";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Wrap R3D";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Wrap R3D") },
        RegValue { name: "Version",     value: String::from("v1.9.0") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Wrap R3D") },
        RegValue { name: "Publisher",       value: String::from("Wrap") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Wrap R3D\\uninstall.exe\"") },
    ]
}
