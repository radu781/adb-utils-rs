use std::process::Command;

use crate::ADBCommand;

pub struct ADBDevices {}

impl ADBCommand for ADBDevices {
    fn build(&self) -> Result<Command, String> {
        let mut shell = Command::new("adb");
        shell.arg("devices");
        Ok(shell)
    }
}
