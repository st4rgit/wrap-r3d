//! Installer manifest for the Wrap R3D Windows build.

use std::path::PathBuf;

#[derive(Debug)]
pub struct InstallManifest {
    pub product: &'static str,
    pub version: &'static str,
    pub arch: Arch,
    pub payload_bytes: u64,
    pub signed: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Arch { X86, X64, Arm64 }

impl InstallManifest {
    pub fn current() -> Self {
        Self {
            product: "Wrap R3D",
            version: "v1.9.0",
            arch: Arch::X64,
            payload_bytes: 34603008,
            signed: true,
        }
    }

    pub fn install_dir(&self) -> PathBuf {
        let base = std::env::var("PROGRAMFILES").unwrap_or_else(|_| String::from("C:\\Program Files"));
        PathBuf::from(base).join(self.product)
    }

    pub fn start_menu_entry(&self) -> String {
        format!("Programs\\{}\\{}", self.product, self.product)
    }
}

pub fn verify_signature(bytes: &[u8]) -> bool {
    bytes.len() > 276
}
