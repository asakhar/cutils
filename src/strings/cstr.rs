use super::common::{common_cstr_impls, common_cstring_impls};

common_cstr_impls!(U8CStr, u8, U8CString, DisplayU8CStr);
common_cstring_impls!(U8CString, u8, U8CStr);
pub type CStr = U8CStr;
pub type CString = U8CString;

#[cfg(not(feature = "no_std"))]
impl std::io::Write for &mut U8CStr {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    let writable = self.0.len() - 1;
    let mut mybuf = &mut self.0[0..writable];
    let written = mybuf.write(buf)?;
    self.0[written] = 0;
    *self = unsafe { std::mem::transmute(&mut self.0[written..]) };
    Ok(written)
  }

  fn flush(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

#[cfg(not(feature = "no_std"))]
impl std::io::Write for U8CString {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    let len = self.refresh();
    if buf.is_empty() {
      return Ok(0);
    }
    let inner = self.inner();
    inner.0.truncate(len);
    let len = inner.0.write(buf)?;
    inner.0.push(0);
    self.refresh();
    Ok(len)
  }

  fn flush(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

impl<'a> From<&'a U8CStr> for &'a core::ffi::CStr {
  fn from(value: &'a U8CStr) -> Self {
    unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(value.as_slice_with_nul()) }
  }
}
impl<'a> From<&'a mut U8CStr> for &'a mut core::ffi::CStr {
  fn from(value: &'a mut U8CStr) -> Self {
    let cstr_ref =
      unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(value.as_slice_with_nul()) };
    unsafe { &mut *(cstr_ref as *const _ as *mut _) }
  }
}
impl From<&core::ffi::CStr> for &U8CStr {
  fn from(value: &core::ffi::CStr) -> Self {
    unsafe { std::mem::transmute(value.to_bytes_with_nul()) }
  }
}
impl From<&mut core::ffi::CStr> for &mut U8CStr {
  fn from(value: &mut core::ffi::CStr) -> Self {
    let slice = value.to_bytes_with_nul();
    let slice: &mut [u8] = unsafe { &mut *(slice as *const _ as *mut _) };
    unsafe { core::mem::transmute(slice) }
  }
}

#[cfg(test)]
mod tests {
  use super::U8CStr;
  use std::io::Write;
  #[test]
  fn test_display() {
    let buf = *b"123\0";
    let cstr: &U8CStr = (&buf).try_into().unwrap();
    assert_eq!("123", format!("{}", cstr.display()));
  }

  #[test]
  fn test_writes_full() {
    let mut buf = *b"123\0";
    let mut cstr: &mut U8CStr = (&mut buf).try_into().unwrap();
    write!(cstr, "456").unwrap();
    assert_eq!(buf, *b"456\0");
    assert_eq!(cstr.as_slice(), b"");
    assert_eq!(cstr.as_slice_with_nul(), b"\0");
    assert_eq!(cstr.as_slice_full(), b"\0");
  }
  #[test]
  fn test_writes1() {
    let mut buf = *b"123\0";
    let mut cstr: &mut U8CStr = (&mut buf).try_into().unwrap();
    write!(cstr, "45").unwrap();
    assert_eq!(buf, *b"45\0\0");
    assert_eq!(cstr.as_slice(), b"");
    assert_eq!(cstr.as_slice_with_nul(), b"\0");
    assert_eq!(cstr.as_slice_full(), b"\0\0");
  }
  #[test]
  fn test_writes_mid_nul() {
    let mut buf = *b"123\0456";
    let mut cstr: &mut U8CStr = (&mut buf).try_into().unwrap();
    write!(cstr, "45").unwrap();
    assert_eq!(buf, *b"45\0\0456");
    assert_eq!(cstr.as_slice(), b"");
    assert_eq!(cstr.as_slice_with_nul(), b"\0");
    assert_eq!(cstr.as_slice_full(), b"\0\0456");
  }
  #[test]
  fn test_writes_over_mid_nul() {
    let mut buf = *b"123\0456";
    let mut cstr: &mut U8CStr = (&mut buf).try_into().unwrap();
    write!(cstr, "4567").unwrap();
    assert_eq!(buf, *b"4567\056");
    assert_eq!(cstr.as_slice(), b"");
    assert_eq!(cstr.as_slice_with_nul(), b"\0");
    assert_eq!(cstr.as_slice_full(), b"\056");
  }
  #[test]
  fn test_writes_continue() {
    let mut buf = *b"123456789\0";
    let mut cstr: &mut U8CStr = (&mut buf).try_into().unwrap();
    write!(cstr, "abc").unwrap();
    write!(cstr, "def").unwrap();
    assert_eq!(buf, *b"abcdef\089\0");
    assert_eq!(cstr.as_slice(), b"");
    assert_eq!(cstr.as_slice_with_nul(), b"\0");
    assert_eq!(cstr.as_slice_full(), b"\089\0");
  }

  #[test]
  fn test_writes_buf_overflow() {
    let mut buf = *b"\0";
    let mut cstr: &mut U8CStr = (&mut buf).try_into().unwrap();
    assert!(matches!(write!(cstr, "a"), Err(err) if err.kind() == std::io::ErrorKind::WriteZero));
  }

  #[test]
  fn test_writes_bufoverflow_once() {
    let mut buf = *b"\0";
    let mut cstr: &mut U8CStr = (&mut buf).try_into().unwrap();
    assert_eq!(cstr.write(b"a").unwrap(), 0);
  }
}
