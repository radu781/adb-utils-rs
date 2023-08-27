/// This example demonstrates how to connect to an Android device using the pin code
/// To run this example, execute:
/// ```sh
/// cargo run --example pair
/// ```

use adb_utils::{manager::ADBManager, networking::ADBPair};

fn main() {
    let manager = ADBManager::new();

    let mut pair = ADBPair::new("192.168.1.132".to_owned(), 42219, 562813);
    let result = manager.execute(&mut pair);
    match result {
        Ok(ok) => println!("Connected: {ok}"),
        Err(err) => println!("Could not connect: {err}"),
    }
}
