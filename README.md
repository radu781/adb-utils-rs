# ADB Utils

Rust implementation of commonly used ADB commands

## Usage

Get all the photos taken on a certain day

```rust
use adb_utils::{manager::ADBManager, shell::ADBList};

fn main() {
    let mut manager = ADBManager::new();
    manager.cwd("/storage/emulated/0/DCIM/Camera");
    if let Err(err) = manager.connect("192.168.1.133", 36415) {
        println!("Could not connect: {err}");
        return;
    }
    let mut list = ADBList::default();
    let files = manager.execute_path_based(&mut list).unwrap().to_vec();
    files
        .iter()
        .filter(|file| file.starts_with("20230827"))
        .for_each(|file| println!("{file}"));
    // 20230827_132733.jpg
    // 20230827_141248.jpg
}
```

More usage examples [here](examples).

## Supported commands

The list of commands is from running `adb help`

### General

- [x] devices
- [x] help
- [x] version

### Networking

- [x] connect
- [x] disconnect
- [x] pair
- [ ] forward
- [ ] ppp TTY
- [ ] reverse
- [ ] mdns check
- [ ] mdns services

### File transfer

- [x] push
- [x] pull
- [ ] sync

### Shell

- [ ] shell
- [ ] emu command

### App installation

- [ ] install
- [ ] install-multiple
- [ ] uninstall

### Debugging

- [ ] bugreport
- [ ] jdwp
- [ ] logcat

### Security

- [ ] disable-verify
- [ ] enable-verify
- [ ] keygen

### Scripting

- [ ] wait-for
- [ ] get-state
- [ ] get-serialno
- [ ] get-devpath
- [ ] remount
- [ ] reboot
- [ ] sideload
- [ ] root
- [ ] unroot
- [ ] usb
- [ ] tcpip

### Internal debugging

- [ ] start-server
- [ ] kill-server
- [ ] reconnect
- [ ] reconnect device
- [ ] reconnect offline

### USB

- [ ] attach
- [ ] detach

### Environment variables

- [ ] adb trace
- [ ] adb vendor keys
- [ ] android serial
- [ ] android log tags
- [ ] adb local transport max port
- [ ] adb mdns auto connect
