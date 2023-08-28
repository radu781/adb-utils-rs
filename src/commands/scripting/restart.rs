use std::{fmt::Display, process::Command};

use crate::{ADBCommand, ADBResult};

/// Reboot the device, defaults to system image
pub struct ADBReboot {
    shell: Command,
}

impl ADBReboot {
    pub fn new() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("reboot");

        ADBReboot { shell: cmd }
    }

    /// Reboot the device to this point
    pub fn to(mut self, method: RebootMethod) -> Self {
        self.shell.arg(method.to_string());
        self
    }
}

impl Default for ADBReboot {
    fn default() -> Self {
        Self::new()
    }
}

impl ADBCommand for ADBReboot {
    fn build(&mut self) -> Result<&mut Command, String> {
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}

pub enum RebootMethod {
    Bootloader,
    Recovery,
    Sideload,
    SideloadAutoReboot,
}

impl Display for RebootMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RebootMethod::Bootloader => "bootloader",
                RebootMethod::Recovery => "recovery",
                RebootMethod::Sideload => "sideload",
                RebootMethod::SideloadAutoReboot => "sideload-auto-reboot",
            }
        )
    }
}

/// Options for the ADB daemon
pub struct ADBDaemon {
    shell: Command,
}

impl ADBDaemon {
    /// Restart adbd with root permissions
    pub fn root() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("root");

        ADBDaemon { shell: cmd }
    }

    /// Restart adbd without root permissions
    pub fn unroot() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("unroot");

        ADBDaemon { shell: cmd }
    }

    /// Restart adbd listening on USB
    pub fn usb() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("usb");

        ADBDaemon { shell: cmd }
    }

    /// Restart adbd listening on TCP on port
    pub fn tcpip(port: u32) -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("tcp").arg(port.to_string());

        ADBDaemon { shell: cmd }
    }
}

impl ADBCommand for ADBDaemon {
    fn build(&mut self) -> Result<&mut Command, String> {
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}
