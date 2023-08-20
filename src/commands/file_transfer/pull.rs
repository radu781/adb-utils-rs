use std::process::Command;

use crate::ADBCommand;

pub struct ADBPull {
    path: Option<String>,
    remote: String,
    local: String,
}

impl ADBPull {
    pub fn new(remote: &str, local: &str) -> Self {
        ADBPull {
            path: None,
            remote: remote.to_owned(),
            local: local.to_owned(),
        }
    }
}

impl ADBCommand for ADBPull {
    fn build(&self) -> Result<Command, String> {
        match &self.path {
            Some(path) => {
                let mut shell = Command::new("adb");
                shell
                    .arg("pull")
                    .arg(format!("{}{}", path, self.remote))
                    .arg(&self.local);
                Ok(shell)
            }
            None => Err("No path specified".to_string()),
        }
    }
}
