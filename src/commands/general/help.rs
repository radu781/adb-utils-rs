use std::process::Command;

use crate::{ADBCommand, ADBResult};

#[derive(Default)]
pub struct ADBHelp {}

impl ADBCommand for ADBHelp {
    fn build(&self) -> Result<Command, String> {
        let mut shell = Command::new("adb");
        shell.arg("help");
        Ok(shell)
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
