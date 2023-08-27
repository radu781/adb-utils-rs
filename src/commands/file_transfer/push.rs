use std::process::Command;

use crate::{ADBPathCommand, ADBResult};

use super::CompressionAlgorithm;

/// Copy local files/directories to device
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

    /// Only push files that are newer on the host than the device
    pub fn sync(mut self) -> Self {
        self.shell.arg("--sync");
        self
    }

    /// Push files to device without storing to the filesystem
    pub fn dry_run(mut self) -> Self {
        self.shell.arg("-n");
        self
    }

    /// Enable compression with the specified algorithm
    pub fn compression(mut self, algorithm: CompressionAlgorithm) -> Self {
        self.shell.arg("-z").arg(algorithm.to_string());
        self
    }

    /// Disable compression
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
