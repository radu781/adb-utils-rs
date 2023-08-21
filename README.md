# ADB Utils

Rust implementation of commonly used ADB commands

## Supported commands

The list of commands is from running `adb help`

### General

- [x] devices
- [x] help
- [x] version

### Networking

- [ ] connect
- [ ] disconnect
- [ ] pair
- [ ] forward
- [ ] ppp TTY
- [ ] reverse
- [ ] mdns check
- [ ] mdns services

### File transfer

- [ ] push
- [ ] pull
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
