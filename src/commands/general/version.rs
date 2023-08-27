use std::process::Command;

use crate::{ADBCommand, ADBResult};

pub struct ADBVersion {
    shell: Command,
}

impl Default for ADBVersion {
    fn default() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("version");

        Self { shell: cmd }
    }
}

impl ADBCommand for ADBVersion {
    fn build(&mut self) -> Result<&mut Command, String> {
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}

pub struct Version {
    pub adb_version: String,
    pub version: String,
    pub install_path: String,
}

impl From<ADBResult> for Version {
    fn from(value: ADBResult) -> Self {
        #[cfg(target_os = "windows")]
        let value = ADBResult {
            data: value.data.replace("\r\n", "\n"),
        };
        let adb_version_index = "Android Debug Bridge version ".len();
        let version_index = value.data.find("Version ").unwrap();
        let install_index = value.data.find("Installed as ").unwrap();

        Self {
            adb_version: value
                .data
                .get(adb_version_index..version_index - 1)
                .unwrap()
                .to_owned(),
            version: value
                .data
                .get(version_index + "Version ".len()..install_index - 1)
                .unwrap()
                .to_owned(),
            install_path: value
                .data
                .get(install_index + "Installed at ".len()..value.data.len())
                .unwrap()
                .to_owned(),
        }
    }
}
