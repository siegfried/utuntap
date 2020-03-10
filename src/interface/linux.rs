use libc::{c_int, c_short, c_ulong, IFNAMSIZ};
use std::io::{self, Result};
use std::mem;
use std::os::unix::io::RawFd;

union RequestUnion {
    flags: c_short,
}

type Name = [u8; IFNAMSIZ];

#[repr(C)]
pub struct Request {
    name: Name,
    union: RequestUnion,
}

#[cfg(all(target_env = "musl"))]
type RequestId = c_int;
#[cfg(all(not(target_env = "musl")))]
type RequestId = c_ulong;

impl Request {
    pub fn with_flags(device_name: Option<String>, flags: c_short) -> Self {
        Request {
            name: name(device_name),
            union: RequestUnion { flags: flags },
        }
    }

    pub fn set_tuntap(mut self, fd: RawFd) -> Result<String> {
        const TUNSETIFF: RequestId = request_code_write!(b'T', 202, mem::size_of::<c_int>());
        self.ioctl(fd, TUNSETIFF)?;
        let filename = {
            let name: Vec<u8> = self.name.iter().take_while(|n| **n != 0).cloned().collect();
            String::from_utf8(name).expect("Incorrect filename")
        };
        Ok(filename)
    }

    fn ioctl(&mut self, fd: RawFd, id: RequestId) -> Result<()> {
        let result = unsafe { libc::ioctl(fd, id, self) };

        if result == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn name(device_name: Option<String>) -> Name {
    let mut name = [0u8; mem::size_of::<Name>()];
    if let Some(device_name) = device_name {
        name[..device_name.len()].clone_from_slice(device_name.as_bytes());
    }
    name
}
