use std::process::Command;

use crate::{ADBCommand, ADBResult};

/// List connected devices
pub struct ADBDevices {
    shell: Command,
}

impl Default for ADBDevices {
    fn default() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("devices");
        Self { shell: cmd }
    }
}

impl ADBDevices {
    pub fn long(mut self) -> Self {
        self.shell.arg("-l");
        self
    }
}

impl ADBCommand for ADBDevices {
    fn build(&mut self) -> Result<&mut Command, String> {
        Ok(&mut self.shell)
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

/// Helper struct to help parse the result from [ADBDevices]
#[derive(Default)]
pub struct DeviceInfo {
    pub product: String,
    pub model: String,
    pub transport_id: u32,
}

impl From<ADBResult> for DeviceInfo {
    fn from(value: ADBResult) -> Self {
        let product_index = match value.data.find("product:") {
            Some(index) => index,
            None => return Self::default(),
        };
        let model_index = match value.data.find("model:") {
            Some(index) => index,
            None => return Self::default(),
        };
        let transport_index = match value.data.find("transport_id:") {
            Some(index) => index,
            None => return Self::default(),
        };

        Self {
            product: value
                .data
                .get(product_index + "product:".len()..model_index - 1)
                .unwrap()
                .to_owned(),
            model: value
                .data
                .get(model_index + "model:".len()..transport_index - 1)
                .unwrap()
                .to_owned(),
            transport_id: value
                .data
                .get(transport_index + "transport_id:".len()..value.data.len() - 1)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        }
    }
}
