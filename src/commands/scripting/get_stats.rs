use std::process::Command;

use crate::{ADBCommand, ADBResult};

/// Get the state of the current device
pub struct ADBGetState {
    shell: Command,
}

impl Default for ADBGetState {
    fn default() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("get-state");

        ADBGetState { shell: cmd }
    }
}

impl ADBCommand for ADBGetState {
    fn build(&mut self) -> Result<&mut Command, String> {
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}

/// Get the device's serial number(connected wired) or ip(connected wireless)
pub struct ADBGetSerialNo {
    shell: Command,
}

impl Default for ADBGetSerialNo {
    fn default() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("get-serialno");

        ADBGetSerialNo { shell: cmd }
    }
}

impl ADBCommand for ADBGetSerialNo {
    fn build(&mut self) -> Result<&mut Command, String> {
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}

/// Get the device's dev path
pub struct ADBGetDevPath {
    shell: Command,
}

impl Default for ADBGetDevPath {
    fn default() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("get-devpath");

        ADBGetDevPath { shell: cmd }
    }
}

impl ADBCommand for ADBGetDevPath {
    fn build(&mut self) -> Result<&mut Command, String> {
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}
