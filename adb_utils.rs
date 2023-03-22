use std::process::Command;

pub struct ADBManager {
    connected: bool,
    path: String,
}
pub struct ADBList {
    path: Option<String>,
}
pub struct ADBMove {
    path: Option<String>,
    from: String,
    to: String,
}
pub struct ADBPush {
    path: Option<String>,
    local: String,
    remote: String,
}
pub struct ADBPull {
    path: Option<String>,
    remote: String,
    local: String,
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

    pub fn shell(&self, cmd: &mut impl ADBCommand) -> Result<ADBResult, String> {
        cmd.path(self.path.clone());
        let mut command = cmd.build()?;
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

    pub fn drop(&mut self) {
        println!("Im in drop");
        if self.connected {
            self.disconnect();
        }
    }
}

trait ADBCommand {
    fn path(&mut self, path: String);
    fn build(&self) -> Result<Command, String>;
}

impl ADBList {
    pub fn new() -> Self {
        ADBList { path: None }
    }
}

impl ADBCommand for ADBList {
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

impl ADBMove {
    pub fn new(from: &str, to: &str) -> Self {
        ADBMove {
            path: None,
            from: from.to_owned(),
            to: to.to_owned(),
        }
    }
}

impl ADBCommand for ADBMove {
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
    fn path(&mut self, path: String) {
        self.path = Some(path);
    }

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

impl ADBResult {
    pub fn to_string(&self) -> &String {
        &self.data
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.data.split("\r\n").map(|x| x.to_string()).collect()
    }
}
