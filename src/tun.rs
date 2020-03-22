use super::Mode;
use std::fs::File;
use std::io::Result;

/// Options and flags which can be used to configure how a tun device file is
/// opened.
///
/// This builder exposes the ability to configure how a [`std::fs::File`][file] is
/// opened and what operations are permitted on the open file, and what `ioctl`
/// operations should be applied to the file.
///
/// Generally speaking, when using `OpenOptions`, you'll first call [`new`],
/// then chain calls to methods to set each option, then call [`open`].
/// This will give you a [`std::io::Result`][result] with a tuple of
/// [`std::fs::File`][file] and filename [`alloc::string::String`][string]
/// inside that you can further operate on.
///
/// [`new`]: struct.OpenOptions.html#method.new
/// [`open`]: struct.OpenOptions.html#method.open
/// [result]: https://doc.rust-lang.org/nightly/std/io/type.Result.html
/// [file]: https://doc.rust-lang.org/nightly/std/io/struct.File.html
/// [string]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html
///
/// # Examples
///
/// Opening device tun0:
///
/// ```no_run
/// use utuntap::tun::OpenOptions;
///
/// let (file, filename) = OpenOptions::new().number(0).open().unwrap();
/// ```
///
/// Opening device tun0 with non-blocking I/O set:
///
/// ```no_run
/// use utuntap::tun::OpenOptions;
///
/// let (file, filename) = OpenOptions::new()
///             .nonblock(true)
///             .number(0)
///             .open()
///             .unwrap();
/// ```
pub struct OpenOptions {
    options: super::OpenOptions,
}

impl OpenOptions {
    /// Creates a blank new set of options ready for configuration.
    ///
    /// All options are initially set to `false` except read and write.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use utuntap::tun::OpenOptions;
    ///
    /// let mut options = OpenOptions::new();
    /// let (file, filename) = options.number(0).open().unwrap();
    /// ```
    pub fn new() -> Self {
        let mut options = super::OpenOptions::new();
        options.mode(Mode::Tun);
        OpenOptions { options }
    }

    /// Sets the option for read access.
    ///
    /// This option, when true, will indicate that the file should be
    /// `read`-able if opened.
    ///
    /// This opiton defaults to `true`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use utuntap::tun::OpenOptions;
    ///
    /// let mut options = OpenOptions::new();
    /// let (file, filename) = options.read(true).write(true).number(0).open().unwrap();
    /// ```
    pub fn read(&mut self, value: bool) -> &mut Self {
        self.options.read(value);
        self
    }

    /// Sets the option for write access.
    ///
    /// This option, when true, will indicate that the file should be
    /// `write`-able if opened.
    ///
    /// This opiton defaults to `true`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use utuntap::tun::OpenOptions;
    ///
    /// let mut options = OpenOptions::new();
    /// let (file, filename) = options.read(true).write(true).number(0).open().unwrap();
    /// ```
    pub fn write(&mut self, value: bool) -> &mut Self {
        self.options.write(value);
        self
    }

    /// Sets the option for non-blocking I/O.
    ///
    /// This option, when true, will indicate that the file should not
    /// block for data to become available.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use utuntap::tun::OpenOptions;
    ///
    /// let mut options = OpenOptions::new();
    /// let (file, filename) = options.nonblock(true).number(0).open().unwrap();
    /// ```
    #[cfg(target_family = "unix")]
    pub fn nonblock(&mut self, value: bool) -> &mut Self {
        self.options.nonblock(value);
        self
    }

    /// Sets the option for device number.
    ///
    /// This option will indicate the number part of the device name,
    /// e.g. 0 of tun0.
    ///
    /// * On Linux, when it is not set, the OS will assign a name for you
    /// * On OpenBSD, it is required, otherwise it will panic
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use utuntap::tun::OpenOptions;
    ///
    /// let mut options = OpenOptions::new();
    /// let (file, filename) = options.number(0).open().unwrap();
    /// ```
    pub fn number(&mut self, value: u8) -> &mut Self {
        self.options.number(value);
        self
    }

    /// Sets the option for packet info.
    ///
    /// This option, when true, will indicate that each packet read or
    /// written is prefixed with a 4-byte packet info.
    ///
    /// This option is only available on Linux.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use utuntap::tun::OpenOptions;
    ///
    /// let mut options = OpenOptions::new();
    /// let (file, filename) = options.packet_info(true).number(0).open().unwrap();
    /// ```
    #[cfg(target_os = "linux")]
    pub fn packet_info(&mut self, value: bool) -> &mut Self {
        self.options.packet_info(value);
        self
    }

    /// Opens a tun device file with the options specified by `self`.
    ///
    /// # Errors
    ///
    /// This function will return an error under a number of different
    /// circumstances. Some of these error conditions are listed here, together
    /// with their [`ErrorKind`]. The mapping to [`ErrorKind`]s is not part of
    /// the compatibility contract of the function, especially the `Other` kind
    /// might change to more specific kinds in the future.
    ///
    /// * [`NotFound`]: The device file does not exist.
    /// * [`PermissionDenied`]: The user lacks permission to get the specified
    ///   access rights for the file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use utuntap::tun::OpenOptions;
    ///
    /// let mut options = OpenOptions::new();
    /// let (file, filename) = options.number(0).open().unwrap();
    /// ```
    ///
    /// [`ErrorKind`]: https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html
    /// [`NotFound`]: https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html#variant.NotFound
    /// [`PermissionDenied`]: https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html#variant.PermissionDenied
    pub fn open(&mut self) -> Result<(File, String)> {
        let (file, filename) = self.options.open()?;
        Ok((file, filename))
    }
}
