# TunTap

A low level Rust library for Tun/Tap devices.

## Usage

It simply provides more options to open Tun/Tap device files.

```rust
use tun_tap::tun;


let (mut file, filename) = tun::OpenOptions::new()
    .packet_info(false) // Only available on Linux
    .number(10)
    .open()
    .expect("failed to open device");
```

## Support Platforms
- [*] Linux
- [ ] OpenBSD
