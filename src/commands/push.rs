use std::process::Command;

use super::ADBCommand;

pub struct ADBPush {
    path: Option<String>,
    local: String,
    remote: String,
}
impl ADBPush {
    pub fn new(local: &str, remote: &str) -> Self {
        ADBPush {
            path: None,
            local: local.to_owned(),
            remote: remote.to_owned(),
        }
    }
}

impl ADBCommand for ADBPush {
    fn path(&mut self, path: String) {
        self.path = Some(path);
    }

    fn build(&self) -> Result<Command, String> {
        match &self.path {
            Some(path) => {
                let mut shell = Command::new("adb");
                shell
                    .arg("push")
                    .arg(&self.local)
                    .arg(format!("{}{}", path, self.remote));
                Ok(shell)
            }
            None => Err("No path specified".to_string()),
        }
    }
}
