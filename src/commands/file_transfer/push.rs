use std::process::Command;

use crate::{ADBPathCommand, ADBResult, CompressionAlgorithm};

pub struct ADBPush {
    path: Option<String>,
    local: String,
    remote: String,
    shell: Command,
}

impl ADBPush {
    pub fn new(local: &str, remote: &str) -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("push");

        ADBPush {
            path: None,
            local: local.to_owned(),
            remote: remote.to_owned(),
            shell: cmd,
        }
    }

    pub fn sync(mut self) -> Self {
        self.shell.arg("--sync");
        self
    }

    pub fn dry_run(mut self) -> Self {
        self.shell.arg("-n");
        self
    }

    pub fn compression(mut self, algorithm: CompressionAlgorithm) -> Self {
        self.shell.arg("-z").arg(algorithm.to_string());
        self
    }

    pub fn no_compression(mut self) -> Self {
        self.shell.arg("-Z");
        self
    }
}

impl ADBPathCommand for ADBPush {
    fn build(&mut self) -> Result<&mut Command, String> {
        match &self.path {
            Some(path) => {
                self.shell
                    .arg(format!("{}\\{}", path, self.local))
                    .arg(&self.remote);
                Ok(&mut self.shell)
            }
            None => Err("No path specified".to_string()),
        }
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }

    fn path(&mut self, path: Option<String>) {
        self.path = path
    }
}
