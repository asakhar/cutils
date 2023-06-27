use crate::strings::{U16CStr, U32CStr};

pub trait Write16 {
  fn write16(&mut self, buf: &[u16]) -> std::io::Result<usize>;
  fn flush16(&mut self) -> std::io::Result<()>;
  fn write16_all(&mut self, mut buf: &[u16]) -> std::io::Result<()> {
    while !buf.is_empty() {
      match self.write16(buf) {
        Ok(0) => {
          return Err(std::io::Error::new(
            std::io::ErrorKind::WriteZero,
            "failed to write whole buffer",
          ));
        }
        Ok(n) => buf = &buf[n..],
        Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
        Err(e) => return Err(e),
      }
    }
    Ok(())
  }
  fn write16_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
    // Create a shim which translates a Write to a fmt::Write and saves
    // off I/O errors. instead of discarding them
    struct Adapter<'a, T: ?Sized + 'a> {
      inner: &'a mut T,
      error: std::io::Result<()>,
    }

    impl<T: Write16 + ?Sized> std::fmt::Write for Adapter<'_, T> {
      fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut s = s.as_bytes();
        let mut buf = [0u16; 128];
        use std::io::Write;
        while !s.is_empty() {
          let mut buf = unsafe { U16CStr::from_mut_slice_unchecked(&mut buf) };
          let len = std::cmp::min(s.len(), buf.capacity_usize());
          match buf.write_all(&s[0..len]) {
            Ok(()) => {}
            Err(e) => {
              self.error = Err(e);
              return Err(std::fmt::Error);
            }
          }
          match self.inner.write16_all(buf.as_slice()) {
            Ok(()) => {}
            Err(e) => {
              self.error = Err(e);
              return Err(std::fmt::Error);
            }
          }
          s = &s[len..];
        }
        Ok(())
      }
    }

    let mut output = Adapter {
      inner: self,
      error: Ok(()),
    };
    match core::fmt::write(&mut output, fmt) {
      Ok(()) => Ok(()),
      Err(..) => {
        // check if the error came from the underlying `Write` or not
        if output.error.is_err() {
          output.error
        } else {
          Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "formatter error",
          ))
        }
      }
    }
  }
  fn by_ref(&mut self) -> &mut Self
  where
    Self: Sized,
  {
    self
  }
}

pub trait Write32 {
  fn write32(&mut self, buf: &[u32]) -> std::io::Result<usize>;
  fn flush32(&mut self) -> std::io::Result<()>;
  fn write32_all(&mut self, mut buf: &[u32]) -> std::io::Result<()> {
    while !buf.is_empty() {
      match self.write32(buf) {
        Ok(0) => {
          return Err(std::io::Error::new(
            std::io::ErrorKind::WriteZero,
            "failed to write whole buffer",
          ));
        }
        Ok(n) => buf = &buf[n..],
        Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
        Err(e) => return Err(e),
      }
    }
    Ok(())
  }
  fn write32_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
    // Create a shim which translates a Write to a fmt::Write and saves
    // off I/O errors. instead of discarding them
    struct Adapter<'a, T: ?Sized + 'a> {
      inner: &'a mut T,
      error: std::io::Result<()>,
    }

    impl<T: Write32 + ?Sized> std::fmt::Write for Adapter<'_, T> {
      fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut s = s.as_bytes();
        let mut buf = [0u32; 128];
        use std::io::Write;
        while !s.is_empty() {
          let mut buf = unsafe { U32CStr::from_mut_slice_unchecked(&mut buf) };
          let len = std::cmp::min(s.len(), buf.capacity_usize());
          match buf.write_all(&s[0..len]) {
            Ok(()) => {}
            Err(e) => {
              self.error = Err(e);
              return Err(std::fmt::Error);
            }
          }
          match self.inner.write32_all(buf.as_slice()) {
            Ok(()) => {}
            Err(e) => {
              self.error = Err(e);
              return Err(std::fmt::Error);
            }
          }
          s = &s[len..];
        }
        Ok(())
      }
    }

    let mut output = Adapter {
      inner: self,
      error: Ok(()),
    };
    match core::fmt::write(&mut output, fmt) {
      Ok(()) => Ok(()),
      Err(..) => {
        // check if the error came from the underlying `Write` or not
        if output.error.is_err() {
          output.error
        } else {
          Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "formatter error",
          ))
        }
      }
    }
  }
  fn by_ref(&mut self) -> &mut Self
  where
    Self: Sized,
  {
    self
  }
}

macro_rules! impl_for_slices {
  ($trait:ident, $type:ty, $write:ident, $flush:ident) => {
    impl $trait for &mut [$type] {
      fn $write(&mut self, buf: &[$type]) -> std::io::Result<usize> {
        let written = std::cmp::min(buf.len(), self.len());
        self.copy_from_slice(&buf[0..written]);
        *self = unsafe {
          std::slice::from_raw_parts_mut(self.as_mut_ptr().add(written), self.len() - written)
        };
        // *self = &mut self[written..];
        Ok(written)
      }

      fn $flush(&mut self) -> std::io::Result<()> {
        Ok(())
      }
    }
  };
}

impl_for_slices!(Write16, u16, write16, flush16);
impl_for_slices!(Write32, u32, write32, flush32);

macro_rules! impl_for_vecs {
  ($trait:ident, $type:ty, $write:ident, $flush:ident) => {
    impl $trait for Vec<$type> {
      fn $write(&mut self, buf: &[$type]) -> std::io::Result<usize> {
        let written = buf.len();
        let old_len = self.len();
        let written = if self.try_reserve_exact(written).is_err() {
          std::cmp::min(self.capacity() - old_len, written)
        } else {
          written
        };
        self.resize(old_len + written, 0);
        self[old_len..old_len + written].copy_from_slice(buf);
        Ok(written)
      }

      fn $flush(&mut self) -> std::io::Result<()> {
        Ok(())
      }
    }
  };
}

impl_for_vecs!(Write16, u16, write16, flush16);
impl_for_vecs!(Write32, u32, write32, flush32);
