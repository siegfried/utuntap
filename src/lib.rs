#[cfg(target_os = "linux")]
#[macro_use]
extern crate nix;

#[cfg(target_os = "linux")]
#[path = "interface/linux.rs"]
mod interface;

#[cfg(target_os = "linux")]
use interface::Flags;
use std::fmt;
use std::fs::{self, File};
use std::io::Result;
#[cfg(target_family = "unix")]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;

#[derive(Debug, PartialEq)]
enum Mode {
    Tun,
    Tap,
}

impl fmt::Display for Mode {
    #[cfg(not(target_os = "macos"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Mode::Tap => "tap",
            Mode::Tun => "tun",
        };
        write!(f, "{}", text)
    }

    #[cfg(target_os = "macos")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Mode::Tap => unimplemented!("TAP is not supported on macOS"),
            Mode::Tun => "utun",
        };
        write!(f, "{}", text)
    }
}

struct OpenOptions {
    options: fs::OpenOptions,
    mode: Mode,
    number: Option<u8>,
    #[cfg(target_family = "unix")]
    nonblock: bool,
    #[cfg(target_os = "linux")]
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
            #[cfg(target_family = "unix")]
            nonblock: false,
            #[cfg(target_os = "linux")]
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

    #[cfg(target_family = "unix")]
    fn nonblock(&mut self, value: bool) -> &mut Self {
        self.nonblock = value;
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

    #[cfg(target_os = "linux")]
    fn packet_info(&mut self, value: bool) -> &mut Self {
        self.packet_info = value;
        self
    }

    #[cfg(target_os = "linux")]
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

    #[cfg(target_family = "unix")]
    fn options(&mut self) -> &fs::OpenOptions {
        if self.nonblock {
            self.options.custom_flags(libc::O_NONBLOCK)
        } else {
            &self.options
        }
    }

    fn device_name(&self) -> Option<String> {
        if let Some(number) = self.number {
            Some(format!("{}{}", self.mode, number))
        } else {
            None
        }
    }

    #[cfg(target_os = "linux")]
    fn open(&mut self) -> Result<(File, String)> {
        let file = self.options().open("/dev/net/tun")?;
        let filename = interface::Request::with_flags(self.device_name(), self.flags())
            .set_tuntap(file.as_raw_fd())?;
        Ok((file, filename))
    }

    #[cfg(target_os = "openbsd")]
    fn open(&mut self) -> Result<(File, String)> {
        if let Some(filename) = self.device_name() {
            let path = std::path::Path::new("/dev").join(&filename);
            let file = self.options().open(path)?;
            Ok((file, filename))
        } else {
            panic!("Unknown device number.")
        }
    }

    #[cfg(target_os = "macos")]
    fn open(&mut self) -> Result<(File, String)> {
        use std::{io::Error, os::fd::FromRawFd, mem, ffi::{c_uchar, c_ushort}};
        use libc::{socket, PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL, ioctl, c_ulong, connect, sockaddr, socklen_t, sockaddr_ctl, getsockopt, UTUN_OPT_IFNAME, c_void, fcntl, F_SETFD, FD_CLOEXEC, F_SETFL, O_NONBLOCK};
        const AF_SYSTEM: c_uchar = 32;
        const AF_SYS_CONTROL: c_ushort = 2;
        const CTLIOCGINFO: c_ulong = 0xc0644e03;
        const UTUN_CONTROL_NAME: &'static str = "com.apple.net.utun_control";

        if let Mode::Tap = self.mode {
            unimplemented!("TAP mode is not supported on macOS")
        }

        let file = {
            let fd = unsafe { socket(PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL) };
            if fd < 0 {
                return Err(Error::last_os_error());
            }

            #[repr(C)]
            pub struct ctl_info {
                pub ctl_id: u32,
                pub ctl_name: [u8; 96],
            }

            let mut info = ctl_info {
                ctl_id: 0,
                ctl_name: {
                    let mut buffer = [0u8; 96];
                    buffer[..UTUN_CONTROL_NAME.len()].clone_from_slice(UTUN_CONTROL_NAME.as_bytes());
                    buffer
                },
            };

            let err = unsafe { ioctl(fd, CTLIOCGINFO, &mut info) };
            if err != 0 {
                return Err(Error::last_os_error());
            }

            let addr = sockaddr_ctl {
                sc_len: mem::size_of::<sockaddr_ctl>() as u8,
                sc_family: AF_SYSTEM,
                ss_sysaddr: AF_SYS_CONTROL,
                sc_id: info.ctl_id,
                // Some says the number is sc_unit, some says it is sc_unit - 1
                sc_unit: u32::from(self.number.expect("missing device number")),
                sc_reserved: [0; 5],
            };

            let err = unsafe {
                let addr_ptr = &addr as *const sockaddr_ctl;
                connect(fd,
                        addr_ptr as *const sockaddr,
                        mem::size_of_val(&addr) as socklen_t)
            };
            if err != 0 {
                return Err(Error::last_os_error());
            }

            let mut name_buf = [0u8; 64];
            let mut name_length: socklen_t = 64;
            let err = unsafe {
                getsockopt(fd,
                           SYSPROTO_CONTROL,
                           UTUN_OPT_IFNAME,
                           &mut name_buf as *mut _ as *mut c_void,
                           &mut name_length as *mut socklen_t)
            };
            if err != 0 {
                return Err(Error::last_os_error());
            }

            let err = unsafe { fcntl(fd, F_SETFD, FD_CLOEXEC) };
            if err != 0 {
                return Err(Error::last_os_error());
            }

            if self.nonblock {
                let err = unsafe { fcntl(fd, F_SETFL, O_NONBLOCK) };
                if err != 0 {
                    return Err(Error::last_os_error());
                }
            }

            unsafe { File::from_raw_fd(fd) }
        };

        Ok((file, self.device_name().unwrap()))
    }
}

pub mod tap;
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

    #[cfg(target_os = "linux")]
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
