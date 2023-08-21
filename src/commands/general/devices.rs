use std::process::Command;

use crate::{ADBCommand, ADBResult};

pub struct ADBDevices {
    flag: Option<Flags>,
}

impl ADBCommand for ADBDevices {
    fn build(&self) -> Result<Command, String> {
        let mut shell = Command::new("adb");
        shell.arg("devices");
        if self.flag.is_some() {
            shell.arg("-l");
        }
        Ok(shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        ADBResult {
            data: output
                .data
                .replace("List of devices attached", "")
                .replace("device", "")
                .replace("\r\n", "\n")
                .replace("\n\n", "\n")
                .replace('\t', ""),
        }
    }
}

#[derive(Clone)]
pub enum Flags {
    Long,
}

impl ADBDevices {
    pub fn new(flags: &Vec<Flags>) -> Self {
        Self {
            flag: if flags.is_empty() {
                None
            } else {
                Some(flags.first().unwrap().clone())
            },
        }
    }
}
