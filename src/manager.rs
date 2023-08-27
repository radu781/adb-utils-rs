use crate::{ADBCommand, ADBPathCommand, ADBResult};
use std::process::Command;

#[derive(Default)]
pub struct ADBManager {
    connected: Vec<(String, u32)>,
    path: Option<String>,
}

impl ADBManager {
    pub fn new() -> ADBManager {
        ADBManager {
            connected: vec![],
            path: None,
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
                    self.connected.push((ip.to_owned(), port));
                    return Ok(());
                }
                Err(s.status.to_string())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn cwd(&mut self, path: &str) {
        if !path.ends_with('/') {
            self.path = Some(path.to_owned() + "/");
        } else {
            self.path = Some(path.to_owned());
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

    fn execute_impl(&self, command: &mut Command) -> Result<ADBResult, String> {
        match command.output() {
            Ok(ok) => {
                if ok.status.success() {
                    Ok(ADBResult {
                        data: String::from_utf8(ok.stdout).unwrap(),
                    })
                } else {
                    Err(ok.status.to_string()
                        + &String::from_utf8(ok.stdout).unwrap()
                        + &String::from_utf8(ok.stderr).unwrap())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn disconnect(&mut self, ip: &str, port: u32) {
        Self::disconnect_one(ip, port);
        if let Some(index) = self
            .connected
            .iter()
            .position(|x| *x == (ip.to_owned(), port))
        {
            self.connected.remove(index);
        }
    }

    fn disconnect_one(ip: &str, port: u32) {
        let mut command = Command::new("adb");
        command
            .arg("disconnect")
            .arg(format!("{ip}:{port}"))
            .output()
            .ok();
    }

    pub fn disconnect_all(&mut self) {
        self.connected
            .iter()
            .for_each(|(ip, port)| Self::disconnect_one(ip, *port));
    }
}

impl Drop for ADBManager {
    fn drop(&mut self) {
        self.disconnect_all();
    }
}
