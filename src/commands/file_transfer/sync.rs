use std::{fmt::Display, process::Command};

use crate::{ADBCommand, ADBResult};

use super::CompressionAlgorithm;

/// Sync a local build from $ANDROID_PRODUCT_OUT to the device
pub struct ADBSync {
    location: SyncLocation,
    shell: Command,
}

impl ADBSync {
    pub fn new(location: SyncLocation) -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("sync");

        ADBSync {
            shell: cmd,
            location,
        }
    }

    /// Push files to device without storing to the filesystem
    pub fn dry_run(mut self) -> Self {
        self.shell.arg("-n");
        self
    }

    /// List files that would be copied, but don't copy them
    pub fn list(mut self) -> Self {
        self.shell.arg("-l");
        self
    }

    /// Enable compression with the specified algorithm
    pub fn compression(mut self, algorithm: CompressionAlgorithm) -> Self {
        self.shell.arg("-z").arg(algorithm.to_string());
        self
    }

    /// Disable compression
    pub fn no_compression(mut self) -> Self {
        self.shell.arg("-Z");
        self
    }
}

impl ADBCommand for ADBSync {
    fn build(&mut self) -> Result<&mut Command, String> {
        self.shell.arg(self.location.to_string());
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}

pub enum SyncLocation {
    All,
    Data,
    Odm,
    Oem,
    Product,
    System,
    SystemExt,
    Vendor,
}

impl Display for SyncLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SyncLocation::All => "all",
                SyncLocation::Data => "data",
                SyncLocation::Odm => "odm",
                SyncLocation::Oem => "oem",
                SyncLocation::Product => "product",
                SyncLocation::System => "system",
                SyncLocation::SystemExt => "system_ext",
                SyncLocation::Vendor => "vendor",
            }
        )
    }
}
