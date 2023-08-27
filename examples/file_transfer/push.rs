/// This example demonstrates how to connect to an Android device and copy a file or directory to it
/// To run this example, execute:
/// ```sh
/// cargo run --example push
/// ```
use adb_utils::manager::ADBManager;
use adb_utils::file_transfer::push::ADBPush;

fn main() {
    let mut manager = ADBManager::new();
    match manager.connect("192.168.1.132", 40785) {
        Ok(()) => println!("Successfully connected"),
        Err(e) => println!("Could not connect: {}", e),
    }

    let mut push = ADBPush::new(
        "C:\\Users\\Radu\\Desktop\\sample.txt",
        "/storage/emulated/0/sample.txt",
    );
    match manager.execute_path_based(&mut push) {
        Ok(ok) => println!("{ok}"),
        Err(err) => println!("error {err}"),
    }
}
