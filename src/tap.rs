//! APIs for level 2 Tap devices

use super::Mode;
use std::fs::File;
use std::io::Result;

/**
Options and flags which can be used to configure how a tap device file is
opened.

This builder exposes the ability to configure how a [`std::fs::File`][file] is
opened and what operations are permitted on the open file, and what `ioctl`
operations should be applied to the file.

Generally speaking, when using `OpenOptions`, you'll first call [`new`],
then chain calls to methods to set each option, then call [`open`].
This will give you a [`std::io::Result`][result] with a tuple of
[`std::fs::File`][file] and filename [`alloc::string::String`][string]
inside that you can further operate on.

[`new`]: struct.OpenOptions.html#method.new
[`open`]: struct.OpenOptions.html#method.open
[result]: https://doc.rust-lang.org/nightly/std/io/type.Result.html
[file]: https://doc.rust-lang.org/nightly/std/io/struct.File.html
[string]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html

# Examples

Opening device `tap0`:

```no_run
use utuntap::tap::OpenOptions;

let file = OpenOptions::new().open(0).unwrap();
```

Opening device `tap0` with non-blocking I/O set:

```no_run
use utuntap::tap::OpenOptions;

let file = OpenOptions::new()
            .nonblock(true)
            .open(0)
            .unwrap();
```
*/
pub struct OpenOptions {
    options: super::OpenOptions,
}

impl OpenOptions {
    /**
    Creates a blank new set of options ready for configuration.

    All options are initially set to `false` except read and write.

    # Examples

    ```no_run
    use utuntap::tap::OpenOptions;

    let mut options = OpenOptions::new();
    let file = options.open(0).unwrap();
    ```
    */
    pub fn new() -> Self {
        let mut options = super::OpenOptions::new();
        options.mode(Mode::Tap);
        Self { options }
    }

    /**
    Sets the option for read access.

    This option, when true, will indicate that the file should be
    `read`-able if opened.

    This opiton defaults to `true`.

    # Examples

    ```no_run
    use utuntap::tap::OpenOptions;

    let mut options = OpenOptions::new();
    let file = options.read(true).write(true).open(0).unwrap();
    ```
    */
    pub fn read(&mut self, value: bool) -> &mut Self {
        self.options.read(value);
        self
    }

    /**
    Sets the option for write access.

    This option, when true, will indicate that the file should be
    `write`-able if opened.

    This opiton defaults to `true`.

    # Examples

    ```no_run
    use utuntap::tap::OpenOptions;

    let mut options = OpenOptions::new();
    let file = options.read(true).write(true).open(0).unwrap();
    ```
    */
    pub fn write(&mut self, value: bool) -> &mut Self {
        self.options.write(value);
        self
    }

    /**
    Sets the option for non-blocking I/O.

    This option, when true, will indicate that the file should not
    block for data to become available.

    # Examples

    ```no_run
    use utuntap::tap::OpenOptions;

    let mut options = OpenOptions::new();
    let file = options.nonblock(true).open(0).unwrap();
    ```
    */
    #[cfg(target_family = "unix")]
    pub fn nonblock(&mut self, value: bool) -> &mut Self {
        self.options.nonblock(value);
        self
    }

    /**
    Sets the option for packet info.

    This option, when true, will indicate that each packet read or
    written is prefixed with a 4-byte packet info.

    This option is only available on Linux.

    # Examples

    ```no_run
    use utuntap::tap::OpenOptions;

    let mut options = OpenOptions::new();
    let file = options.packet_info(true).open(0).unwrap();
    ```
    */
    #[cfg(target_os = "linux")]
    pub fn packet_info(&mut self, value: bool) -> &mut Self {
        self.options.packet_info(value);
        self
    }

    /**
    Opens a tap device file with the options specified by `self`.

    # Arguments

    * `number` - the number of the device, e.g. the "0" of "tun0".

    # Errors

    This function will return an error under a number of different
    circumstances. Some of these error conditions are listed here, together
    with their [`ErrorKind`]. The mapping to [`ErrorKind`]s is not part of
    the compatibility contract of the function.

    * [`NotFound`]: The device file does not exist.
    * [`PermissionDenied`]: The user lacks permission to get the specified
      access rights for the file.

    # Examples

    ```no_run
    use utuntap::tap::OpenOptions;

    let mut options = OpenOptions::new();
    let file = options.open(0).unwrap();
    ```

    [`ErrorKind`]: https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html
    [`NotFound`]: https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html#variant.NotFound
    [`PermissionDenied`]: https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html#variant.PermissionDenied
    */
    pub fn open(&mut self, number: u32) -> Result<File> {
        Ok(self.options.open(number)?)
    }
}
