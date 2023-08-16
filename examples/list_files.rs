/// This example demonstrates how to connect to an Android device and print all the contents of a folder
/// To run this example, execute:
/// ```sh
/// cargo run --example list_files
/// ```

use adb_utils::{ADBManager, ADBList};

fn main() {
    let mut manager = ADBManager::new();
    match manager.connect("192.168.1.129", "42457") {
        Ok(())=> println!("Successfully connected"),
        Err(e) => println!("Could not connect: {}", e)
    }
    manager.cwd("/storage/emulated/0");

    let mut list = ADBList::default();
    match manager.shell(&mut list) {
        Ok(ok) => println!("{}", ok.to_string()),
        Err(err) => println!("Error: {}", err)
    }
}
