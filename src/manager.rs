use crate::{ADBCommand, ADBResult, ADBPathCommand};
use std::process::Command;

#[derive(Default)]
pub struct ADBManager {
    connected: bool,
    path: String,
}

impl ADBManager {
    pub fn new() -> ADBManager {
        ADBManager {
            connected: false,
            path: "".to_string(),
        }
    }

    pub fn connect(&mut self, ip: &str, port: u32) -> Result<(), String> {
        let mut command = Command::new("cmd");
        command
            .arg("/c")
            .arg(format!("adb connect {}:{}", ip, port));
        match command.output() {
            Ok(s) => {
                if s.status.success() && !String::from_utf8(s.stdout).unwrap().contains("failed") {
                    self.connected = true;
                    return Ok(());
                }
                Err(s.status.to_string())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn cwd(&mut self, path: &str) {
        if !path.ends_with('/') {
            self.path = path.to_owned() + "/";
        } else {
            self.path = path.to_owned();
        }
    }

    pub fn execute(&self, cmd: &mut impl ADBCommand) -> Result<ADBResult, String> {
        let command = cmd.build()?;
        let result = self.execute_impl(command)?;
        Ok(cmd.process_output(result))
    }

    pub fn execute_path_based(&self, cmd: &mut impl ADBPathCommand) -> Result<ADBResult, String> {
        cmd.path(self.path.clone());
        let command = cmd.build()?;
        let result = self.execute_impl(command)?;
        Ok(cmd.process_output(result))
    }

    fn execute_impl(&self, mut command: Command) -> Result<ADBResult, String> {
        match command.output() {
            Ok(ok) => {
                if ok.status.success() {
                    Ok(ADBResult {
                        data: String::from_utf8(ok.stdout).unwrap(),
                    })
                } else {
                    Err(ok.status.to_string() + &String::from_utf8(ok.stderr).unwrap())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn disconnect(&mut self) {
        let mut command = Command::new("cmd");
        command.arg("/c").arg("adb disconnect").output().ok();
        self.connected = false;
    }
}

impl Drop for ADBManager {
    fn drop(&mut self) {
        if self.connected {
            self.disconnect();
        }
    }
}
