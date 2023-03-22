use std::process::Command;

pub struct ADBManager {
    connected: bool,
    path: String,
}

pub enum ADBCommand {
    List,
    Move,
    Push,
    Pull,
}

pub struct ADBResult {
    data: String,
}

impl ADBManager {
    pub fn new() -> ADBManager {
        ADBManager {
            connected: false,
            path: "".to_string(),
        }
    }

    pub fn connect(&mut self, ip: &str, port: &str) -> Result<(), String> {
        let mut command = Command::new("cmd");
        command
            .arg("/c")
            .arg(format!("adb connect {}:{}", ip, port));
        match command.output() {
            Ok(s) => {
                if s.status.success() {
                    self.connected = true;
                    return Ok(());
                }
                return Err(s.status.to_string());
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn cwd(&mut self, path: &String) {
        if !path.ends_with("/") {
            self.path = path.clone() + "/";
        } else {
            self.path = path.clone();
        }
    }

    pub fn shell(&self, cmd: &ADBCommand, params: &Vec<String>) -> Result<ADBResult, String> {
        let mut command = self.build_command(cmd, params)?;
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

    fn build_command(&self, cmd: &ADBCommand, params: &Vec<String>) -> Result<Command, String> {
        let mut shell = Command::new("adb");
        shell.arg("shell");
        let mut simple_shell = Command::new("adb");
        match cmd {
            ADBCommand::List => {
                if params.len() > 0 {
                    shell.arg(format!("ls {}{}", self.path, params[0]))
                } else {
                    shell.arg(format!("ls {}", self.path))
                }
            }
            ADBCommand::Move => {
                if params.len() != 2 {
                    return Err("Expected 2 parameters for move".to_string());
                }
                shell.arg(format!(
                    "mv {}{} {}{}",
                    self.path, params[0], self.path, params[1]
                ))
            }
            ADBCommand::Push => {
                if params.len() != 2 {
                    return Err("Expected 2 parameters for push".to_string());
                }
                simple_shell.arg(format!("push {} {}", params[0], params[1]))
            }
            ADBCommand::Pull => {
                if params.len() != 2 {
                    return Err("Expected 2 parameters for pull".to_string());
                }
                simple_shell.arg("pull");
                simple_shell.arg(format!("{}{}", self.path, params[0]));
                simple_shell.arg(&params[1])
            }
        };
        if shell.get_args().len() == 1{
            return Ok(simple_shell);
        }
        return Ok(shell);
    }

    pub fn disconnect(&mut self) {
        let mut command = Command::new("cmd");
        command.arg("/c").arg("adb disconnect").output().ok();
        self.connected = false;
    }

    pub fn drop(&mut self) {
        println!("Im in drop");
        if self.connected {
            self.disconnect();
        }
    }
}

impl ADBResult {
    pub fn to_string(self) -> String {
        self.data
    }

    pub fn to_vec(self) -> Vec<String> {
        self.data.split("\r\n").map(|x| x.to_string()).collect()
    }
}
