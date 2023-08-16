use std::process::Command;

mod list;
mod manager;
mod mv;
mod pull;
mod push;

pub use list::*;
pub use manager::*;
pub use mv::*;
pub use pull::*;
pub use push::*;

pub trait ADBCommand {
    fn path(&mut self, path: String);
    fn build(&self) -> Result<Command, String>;
}

pub struct ADBResult {
    data: String,
}

impl ADBResult {
    pub fn to_string(&self) -> &String {
        &self.data
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.data.split("\r\n").map(|x| x.to_string()).collect()
    }
}
