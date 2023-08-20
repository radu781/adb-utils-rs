/// This example demonstrates how to connect to an Android device and use the shell command to print all the contents of a folder
/// To run this example, execute:
/// ```sh
/// cargo run --example list_files
/// ```

use adb_utils::shell::ADBList;
use adb_utils::manager::ADBManager;

fn main() {
    let mut manager = ADBManager::new();
    match manager.connect("192.168.0.105", 41887) {
        Ok(()) => println!("Successfully connected"),
        Err(e) => println!("Could not connect: {}", e),
    }
    manager.cwd("/storage/emulated/0");

    let mut list = ADBList::default();
    match manager.execute_path_based(&mut list) {
        Ok(ok) => println!("{}", ok.to_string()),
        Err(err) => println!("Error: {}", err),
    }
}
