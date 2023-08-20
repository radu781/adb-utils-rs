use std::process::Command;

use crate::{ADBPathCommand, ADBResult};

pub struct ADBMove {
    path: Option<String>,
    from: String,
    to: String,
}

impl ADBMove {
    pub fn new(from: &str, to: &str) -> Self {
        ADBMove {
            path: None,
            from: from.to_owned(),
            to: to.to_owned(),
        }
    }
}

impl ADBPathCommand for ADBMove {
    fn path(&mut self, path: String) {
        self.path = Some(path);
    }

    fn build(&self) -> Result<Command, String> {
        match &self.path {
            Some(path) => {
                let mut shell = Command::new("adb");
                shell.arg("shell");
                shell.arg(format!("mv {}{} {}{}", path, self.from, path, self.to));
                Ok(shell)
            }
            None => Err("No path specified".to_string()),
        }
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}
