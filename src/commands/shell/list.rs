use std::process::Command;

use crate::{ADBPathCommand, ADBResult};

#[derive(Default)]
pub struct ADBList {
    path: Option<String>,
}

impl ADBPathCommand for ADBList {
    fn path(&mut self, path: Option<String>) {
        self.path = path
    }

    fn build(&mut self) -> Result<&mut Command, String> {
        match &self.path {
            Some(path) => {
                let mut shell = Command::new("adb");
                shell.arg("shell");
                // if params.len() > 0 {
                //     shell.arg(format!("ls {}{}", path, params[0]));
                // } else {
                shell.arg(format!("ls {}", path));
                // }
                todo!()
                // Ok(&mut shell)
            }
            None => Err("No path specified".to_string()),
        }
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}
