use std::process::Command;

use crate::{ADBCommand, ADBResult};

pub struct ADBHelp {
    shell: Command,
}

impl ADBHelp {
    fn new() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("help");

        Self { shell: cmd }
    }
}
impl ADBCommand for ADBHelp {
    fn build(&mut self) -> Result<&mut Command, String> {
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        ADBResult {
            data: output
                .data
                .get(output.data.find("global options").unwrap()..output.data.len())
                .unwrap()
                .to_owned(),
        }
    }
}
