use crate::strings::{U16CStr, U32CStr};

pub trait Write16: super::io::Write16 {
  fn write16_str(&mut self, buf: &U16CStr) -> core::fmt::Result;
  fn write16_char(&mut self, c: u16) -> core::fmt::Result {
    self.write16_str(unsafe { U16CStr::from_slice_unchecked(&[c, 0]) })
  }
}

pub trait Write32: super::io::Write32 {
  fn write32_str(&mut self, buf: &U32CStr) -> core::fmt::Result;
  fn write32_char(&mut self, c: u32) -> core::fmt::Result {
    self.write32_str(unsafe { U32CStr::from_slice_unchecked(&[c, 0]) })
  }
}
