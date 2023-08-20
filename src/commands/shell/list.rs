use std::process::Command;

use crate::ADBPathCommand;

#[derive(Default)]
pub struct ADBList {
    path: Option<String>,
}

impl ADBPathCommand for ADBList {
    fn path(&mut self, path: String) {
        self.path = Some(path);
    }

    fn build(&self) -> Result<Command, String> {
        match &self.path {
            Some(path) => {
                let mut shell = Command::new("adb");
                shell.arg("shell");
                // if params.len() > 0 {
                //     shell.arg(format!("ls {}{}", path, params[0]));
                // } else {
                shell.arg(format!("ls {}", path));
                // }
                Ok(shell)
            }
            None => Err("No path specified".to_string()),
        }
    }
}
