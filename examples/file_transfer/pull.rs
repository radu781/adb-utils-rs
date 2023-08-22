/// This example demonstrates how to connect to an Android device and copy a file or directory from it
/// To run this example, execute:
/// ```sh
/// cargo run --example pull
/// ```
use adb_utils::manager::ADBManager;
use adb_utils::file_transfer::pull::ADBPull;

fn main() {
    let mut manager = ADBManager::new();
    match manager.connect("192.168.0.105", 33363) {
        Ok(()) => println!("Successfully connected"),
        Err(e) => println!("Could not connect: {}", e),
    }

    let mut pull = ADBPull::new("/storage/emulated/0/DCIM/IMG_00000001.jpg", ".");
    match manager.execute_path_based(&mut pull) {
        Ok(ok) => println!("{ok}"),
        Err(err) => println!("error {err}"),
    }
}
