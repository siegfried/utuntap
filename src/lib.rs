#[cfg(target_os = "linux")]
#[macro_use]
extern crate nix;

#[cfg(target_family = "unix")]
use libc::O_NONBLOCK;
use std::fs::File;
use std::io::Result;

#[derive(Debug, PartialEq)]
enum Mode {
    Tun,
    #[cfg(not(target_os = "macos"))]
    Tap,
}

#[cfg(not(target_os = "macos"))]
impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Mode::Tun => "tun",
            Mode::Tap => "tap",
        };
        write!(f, "{}", text)
    }
}

struct OpenOptions {
    mode: Mode,
    read: bool,
    write: bool,
    #[cfg(target_family = "unix")]
    nonblock: bool,
    #[cfg(target_os = "linux")]
    packet_info: bool,
}

impl OpenOptions {
    fn new() -> Self {
        Self {
            mode: Mode::Tun,
            read: true,
            write: true,
            #[cfg(target_family = "unix")]
            nonblock: false,
            #[cfg(target_os = "linux")]
            packet_info: false,
        }
    }

    fn read(&mut self, enabled: bool) -> &mut Self {
        self.read = enabled;
        self
    }

    fn write(&mut self, enabled: bool) -> &mut Self {
        self.write = enabled;
        self
    }

    #[cfg(target_family = "unix")]
    fn nonblock(&mut self, enabled: bool) -> &mut Self {
        self.nonblock = enabled;
        self
    }

    fn mode(&mut self, mode: Mode) -> &mut Self {
        self.mode = mode;
        self
    }

    #[cfg(target_os = "linux")]
    fn packet_info(&mut self, enabled: bool) -> &mut Self {
        self.packet_info = enabled;
        self
    }

    #[cfg(target_os = "linux")]
    fn open(&mut self, number: u32) -> Result<File> {
        use std::{
            io::Error,
            os::unix::{fs::OpenOptionsExt, io::AsRawFd},
        };

        let file = {
            let mut options = std::fs::OpenOptions::new();

            options.read(self.read).write(self.write);
            if self.nonblock {
                options.custom_flags(O_NONBLOCK);
            }

            options.open("/dev/net/tun")?
        };

        use libc::{__c_anonymous_ifr_ifru, c_int, c_short, ifreq, ioctl, strcpy};
        use std::{ffi::CString, mem};

        const IFF_TUN: c_short = 0x0001;
        const IFF_TAP: c_short = 0x0002;
        const IFF_NO_PI: c_short = 0x1000;
        #[cfg(target_env = "musl")]
        type RequestId = c_int;
        #[cfg(not(target_env = "musl"))]
        type RequestId = libc::c_ulong;
        const TUNSETIFF: RequestId = request_code_write!(b'T', 202, mem::size_of::<c_int>());

        let mut request = ifreq {
            ifr_name: Default::default(),
            ifr_ifru: __c_anonymous_ifr_ifru {
                ifru_flags: {
                    let mut flags = match self.mode {
                        Mode::Tun => IFF_TUN,
                        Mode::Tap => IFF_TAP,
                    };
                    if !self.packet_info {
                        flags |= IFF_NO_PI;
                    }
                    flags
                },
            },
        };

        let device_name = CString::new(format!("{}{}", self.mode, number))?;

        unsafe {
            strcpy(request.ifr_name.as_mut_ptr(), device_name.as_ptr());
        }

        let err = unsafe { ioctl(file.as_raw_fd(), TUNSETIFF, &mut request) };
        if err != 0 {
            return Err(Error::last_os_error());
        }

        Ok(file)
    }

    #[cfg(target_os = "openbsd")]
    fn open(&mut self, number: u32) -> Result<File> {
        use std::os::unix::fs::OpenOptionsExt;

        let file = {
            let mut options = std::fs::OpenOptions::new();

            let filename = format!("{}{}", self.mode, number);

            options.read(self.read).write(self.write);
            if self.nonblock {
                options.custom_flags(O_NONBLOCK);
            }

            let path = std::path::Path::new("/dev").join(&filename);
            options.open(path)?
        };

        Ok(file)
    }

    #[cfg(target_os = "macos")]
    fn open(&mut self, number: u32) -> Result<File> {
        use libc::{
            c_ulong, connect, fcntl, ioctl, sockaddr, sockaddr_ctl, socket, socklen_t, FD_CLOEXEC,
            F_SETFD, F_SETFL, PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL,
        };
        use std::{
            ffi::{c_uchar, c_ushort},
            io::Error,
            mem,
            os::fd::FromRawFd,
        };
        const AF_SYSTEM: c_uchar = 32;
        const AF_SYS_CONTROL: c_ushort = 2;
        const CTLIOCGINFO: c_ulong = 0xc0644e03;
        const UTUN_CONTROL_NAME: &'static str = "com.apple.net.utun_control";

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
                    buffer[..UTUN_CONTROL_NAME.len()]
                        .clone_from_slice(UTUN_CONTROL_NAME.as_bytes());
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
                sc_unit: number + 1, // Real device number = sc_unit - 1
                sc_reserved: [0; 5],
            };

            let err = unsafe {
                let addr_ptr = &addr as *const sockaddr_ctl;
                connect(
                    fd,
                    addr_ptr as *const sockaddr,
                    mem::size_of_val(&addr) as socklen_t,
                )
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

        Ok(file)
    }
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(target_os = "macos"))]
pub mod tap;
pub mod tun;
