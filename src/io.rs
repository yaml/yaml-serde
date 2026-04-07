//! I/O traits abstracted over std and no_std.
//!
//! In std mode, re-exports from `std::io`.
//! In no_std mode, provides a minimal `Write` trait and error type.
//!
//! This follows the same pattern as `serde_json`:
//! <https://github.com/serde-rs/json/blob/master/src/io/mod.rs>

#[cfg(feature = "std")]
pub use std::io::{Error, Read, Write};

#[cfg(feature = "std")]
pub(crate) fn sink() -> std::io::Sink {
    std::io::sink()
}

#[cfg(not(feature = "std"))]
pub use self::nostd::{Error, Write};

#[cfg(not(feature = "std"))]
pub(crate) use self::nostd::sink;

#[cfg(not(feature = "std"))]
mod nostd {
    use alloc::vec::Vec;
    use core::fmt;

    pub type Result<T> = core::result::Result<T, Error>;

    /// I/O error type for no_std mode.
    pub struct Error;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("I/O error")
        }
    }

    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("io::Error")
        }
    }

    impl core::error::Error for Error {}

    /// Minimal `Write` trait for no_std environments.
    pub trait Write {
        /// Write all bytes from `buf` into the writer.
        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        /// Flush buffered data.
        fn flush(&mut self) -> Result<()>;
    }

    impl<W: Write + ?Sized> Write for &mut W {
        #[inline]
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            (**self).write_all(buf)
        }

        #[inline]
        fn flush(&mut self) -> Result<()> {
            (**self).flush()
        }
    }

    pub fn sink() -> Sink {
        Sink
    }

    pub struct Sink;

    impl Write for Sink {
        #[inline]
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
            Ok(())
        }

        #[inline]
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    impl Write for Vec<u8> {
        #[inline]
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.extend_from_slice(buf);
            Ok(())
        }

        #[inline]
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
}
