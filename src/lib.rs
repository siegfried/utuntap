#[macro_use]
extern crate nix;

#[cfg_attr(target_os = "linux", path = "interface/linux.rs")]
mod interface;

use std::fmt;
use std::fs::{self, File};
use std::io::Result;
use std::os::unix::io::AsRawFd;
#[cfg(all(target_os = "linux"))]
use interface::Flags;

#[derive(Debug, PartialEq)]
enum Mode {
    Tun,
    Tap,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Mode::Tap => "tap",
            Mode::Tun => "tun",
        };
        write!(f, "{}", text)
    }
}

struct OpenOptions {
    options: fs::OpenOptions,
    mode: Mode,
    number: Option<u8>,
    #[cfg(all(target_os = "linux"))]
    packet_info: bool,
}

impl OpenOptions {
    fn new() -> Self {
        let mut options = fs::OpenOptions::new();
        options.read(true).write(true);
        OpenOptions {
            options,
            mode: Mode::Tun,
            number: None,
            #[cfg(all(target_os = "linux"))]
            packet_info: false,
        }
    }

    fn read(&mut self, value: bool) -> &mut Self {
        self.options.read(value);
        self
    }

    fn write(&mut self, value: bool) -> &mut Self {
        self.options.write(value);
        self
    }

    fn mode(&mut self, value: Mode) -> &mut Self {
        self.mode = value;
        self
    }

    fn number(&mut self, value: u8) -> &mut Self {
        self.number = Some(value);
        self
    }

    #[cfg(all(target_os = "linux"))]
    fn packet_info(&mut self, value: bool) -> &mut Self {
        self.packet_info = value;
        self
    }

    #[cfg(all(target_os = "linux"))]
    fn flags(&self) -> Flags {
        const IFF_TUN: Flags = 0x0001;
        const IFF_TAP: Flags = 0x0002;
        const IFF_NO_PI: Flags = 0x1000;

        let mut flags = match self.mode {
            Mode::Tun => IFF_TUN,
            Mode::Tap => IFF_TAP,
        };
        if !self.packet_info {
            flags |= IFF_NO_PI;
        }
        flags
    }

    fn device_name(&self) -> Option<String> {
        if let Some(number) = self.number {
            Some(format!("{}{}", self.mode, number))
        } else {
            None
        }
    }

    #[cfg(all(target_os = "linux"))]
    fn open(&self) -> Result<(File, String)> {
        let file = self.options.open("/dev/net/tun")?;
        let filename = interface::Request::with_flags(self.device_name(), self.flags())
            .set_tuntap(file.as_raw_fd())?;
        Ok((file, filename))
    }
}

pub mod tun;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_mode() {
        let mut options = OpenOptions::new();
        assert_eq!(options.mode, Mode::Tun);
        options.mode(Mode::Tap);
        assert_eq!(options.mode, Mode::Tap);
        options.mode(Mode::Tun);
        assert_eq!(options.mode, Mode::Tun);
    }

    #[test]
    fn change_number() {
        let mut options = OpenOptions::new();
        assert_eq!(options.number, None);
        options.number(1);
        assert_eq!(options.number, Some(1));
        options.number(2);
        assert_eq!(options.number, Some(2));
    }

    #[cfg(all(target_os = "linux"))]
    #[test]
    fn turn_on_packet_info() {
        let mut options = OpenOptions::new();
        assert_eq!(options.packet_info, false);
        options.packet_info(true);
        assert_eq!(options.packet_info, true);
        options.packet_info(false);
        assert_eq!(options.packet_info, false);
    }

    #[test]
    fn display_device_name() {
        let mut options = OpenOptions::new();
        assert_eq!(options.device_name(), None);
        options.mode(Mode::Tun);
        options.number(0);
        assert_eq!(options.device_name(), Some("tun0".into()));
        options.mode(Mode::Tap);
        assert_eq!(options.device_name(), Some("tap0".into()));
    }
}
