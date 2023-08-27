use std::process::Command;

use crate::{ADBPathCommand, ADBResult};

pub struct ADBList {
    path: Option<String>,
    shell: Command,
}

impl Default for ADBList {
    fn default() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("shell").arg("ls");

        ADBList {
            path: None,
            shell: cmd,
        }
    }
}

impl ADBPathCommand for ADBList {
    fn path(&mut self, path: Option<String>) {
        self.path = path
    }

    fn build(&mut self) -> Result<&mut Command, String> {
        match &self.path {
            Some(path) => {
                self.shell.arg(path);
                Ok(&mut self.shell)
            }
            None => Ok(&mut self.shell),
        }
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}
