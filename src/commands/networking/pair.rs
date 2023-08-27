use std::process::Command;

use crate::{ADBCommand, ADBResult};

pub struct ADBPair {
    ip: String,
    port: u32,
    pin: u32,
    shell: Command,
}

impl ADBPair {
    pub fn new(ip: String, port: u32, pin: u32) -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("pair");
        ADBPair {
            ip,
            port,
            pin,
            shell: cmd,
        }
    }
}

impl ADBCommand for ADBPair {
    fn build(&mut self) -> Result<&mut Command, String> {
        self.shell
            .arg(format!("{}:{}", self.ip, self.port))
            .arg(self.pin.to_string());
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}
