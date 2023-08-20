use std::process::Command;

pub mod app_installation;
pub mod file_transfer;
pub mod general;
pub mod shell;

pub trait ADBCommand {
    fn build(&self) -> Result<Command, String>;
    fn process_output(&self, output: ADBResult) -> ADBResult;
}

pub trait ADBPathCommand {
    fn path(&mut self, path: String);
    fn build(&self) -> Result<Command, String>;
    fn process_output(&self, output: ADBResult) -> ADBResult;
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
