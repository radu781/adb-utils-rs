use std::process::Command;

use crate::{ADBCommand, ADBResult};

#[derive(Default)]
pub struct ADBDevices {}

impl ADBCommand for ADBDevices {
    fn build(&self) -> Result<Command, String> {
        let mut shell = Command::new("adb");
        shell.arg("devices");
        Ok(shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        ADBResult {
            data: output
                .data
                .replace("List of devices attached", "")
                .replace("device", "")
                .replace("\r\n", "\n")
                .replace("\n\n", "\n")
                .replace('\t', "")
        }
    }
}
