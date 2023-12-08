use std::process::Command;

use crate::{ADBCommand, ADBResult};

/// https://en.wikipedia.org/wiki/Multicast_DNS
pub struct ADBMdns {
    shell: Command,
}

impl ADBMdns {
    /// Check if mdns discovery is available
    pub fn check() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("check");

        ADBMdns { shell: cmd }
    }

    /// List all discovered services
    pub fn services() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("services");

        ADBMdns { shell: cmd }
    }
}

impl ADBCommand for ADBMdns {
    fn build(&mut self) -> Result<&mut Command, String> {
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}
