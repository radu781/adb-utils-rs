use std::{fmt::Display, process::Command};

use crate::{ADBCommand, ADBResult};

/// Wait for the device to be in the given state
pub struct ADBWait {
    shell: Command,
}

impl ADBWait {
    pub fn new(transport: Transport, state: State) -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg(format!("wait-for-{}-{}", transport, state));

        ADBWait { shell: cmd }
    }
}

pub enum Transport {
    Usb,
    Local,
    Any,
}

impl Display for Transport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Transport::Usb => "usb",
                Transport::Local => "local",
                Transport::Any => "any",
            }
        )
    }
}

pub enum State {
    Bootloader,
    Device,
    Disconnect,
    Recovery,
    Rescue,
    Sideload,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Bootloader => "bootloader",
                State::Device => "device",
                State::Disconnect => "disconnect",
                State::Recovery => "recovery",
                State::Rescue => "rescue",
                State::Sideload => "sideload",
            }
        )
    }
}

impl ADBCommand for ADBWait {
    fn build(&mut self) -> Result<&mut Command, String> {
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}
