use std::process::Command;

pub mod shell;
pub mod app_installation;
pub mod general;
pub mod file_transfer;

pub trait ADBCommand {
    fn build(&self) -> Result<Command, String>;
}

pub trait ADBPathCommand {
    fn path(&mut self, path: String);
    fn build(&self) -> Result<Command, String>;
}

pub struct ADBResult {
    pub(crate) data: String,
}

impl ADBResult {
    pub fn to_string(&self) -> &String {
        &self.data
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.data.split("\r\n").map(|x| x.to_string()).collect()
    }
}
