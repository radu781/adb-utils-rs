use std::process::Command;

use crate::{ADBPathCommand, ADBResult};

pub struct ADBPull {
    path: Option<String>,
    remote: String,
    local: String,
    shell: Command,
}

impl ADBPull {
    pub fn new(remote: &str, local: &str) -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("pull");

        ADBPull {
            path: None,
            remote: remote.to_owned(),
            local: local.to_owned(),
            shell: cmd,
        }
    }
}

impl ADBPathCommand for ADBPull {
    fn build(&mut self) -> Result<&mut Command, String> {
        match &self.path {
            Some(path) => {
                self.shell
                    .arg(format!("{}{}", path, self.remote))
                    .arg(&self.local);
                Ok(&mut self.shell)
            }
            None => Err("No path specified".to_string()),
        }
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }

    fn path(&mut self, path: String) {
        self.path = Some(path)
    }
}
