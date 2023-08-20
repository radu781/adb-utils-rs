/// This example demonstrates how to connect to an Android device and get general information
/// about the connected devices, adb help and version
/// To run this example, execute:
/// ```sh
/// cargo run --example general_info
/// ```

use adb_utils::general::ADBDevices;
use adb_utils::general::ADBHelp;
use adb_utils::general::ADBVersion;
use adb_utils::general::Version;
use adb_utils::manager::ADBManager;

fn main() {
    let mut manager = ADBManager::new();
    match manager.connect("192.168.0.105", 35659) {
        Ok(()) => println!("Successfully connected"),
        Err(e) => println!("Could not connect: {}", e),
    }

    let mut list = ADBDevices::default();
    match manager.execute(&mut list) {
        Ok(ok) => println!("{}", ok.to_string()),
        Err(err) => println!("Error: {}", err),
    }

    let mut help = ADBHelp::default();
    println!("{}", manager.execute(&mut help).unwrap().to_string());

    let mut version = ADBVersion::default();
    let version = Version::from(manager.execute(&mut version).unwrap());
    println!("adb version: {}, version: {}, install: {}", version.adb_version, version.version, version.install_path);
}
