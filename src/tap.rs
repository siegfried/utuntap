use super::Mode;
use std::fs::File;
use std::io::Result;

pub struct OpenOptions {
    options: super::OpenOptions,
}

impl OpenOptions {
    pub fn new() -> Self {
        let mut options = super::OpenOptions::new();
        options.mode(Mode::Tap);
        OpenOptions { options }
    }

    pub fn read(&mut self, value: bool) -> &mut Self {
        self.options.read(value);
        self
    }

    pub fn write(&mut self, value: bool) -> &mut Self {
        self.options.write(value);
        self
    }

    pub fn number(&mut self, value: u8) -> &mut Self {
        self.options.number(value);
        self
    }

    #[cfg(target_os = "linux")]
    pub fn packet_info(&mut self, value: bool) -> &mut Self {
        self.options.packet_info(value);
        self
    }

    pub fn open(&mut self) -> Result<(File, String)> {
        let (file, filename) = self.options.open()?;
        Ok((file, filename))
    }
}
