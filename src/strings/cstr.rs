use super::{common::{common_cstr_impls, common_cstring_impls, common_staticcstr_impls, common_owningcstr_impls}, internals::{decode_u8, encode_u8}};

common_cstr_impls!(U8CStr, u8, U8CString, DisplayU8CStr, U8CStrIter, StaticU8CStr);
common_staticcstr_impls!(StaticU8CStr, u8, U8CString, U8CStr, DisplayU8CStr, StaticU8CStrIter, super::internals::encode_u8);
common_cstring_impls!(U8CString, u8, U8CStr, DisplayU8CStr, U8CStringIter, super::internals::encode_u8);
common_owningcstr_impls!(U8OwningCStr, u8, U8CString, U8CStr, DisplayU8CStr, U8OwningCStrIter);
pub type CStr = U8CStr;
pub type CString = U8CString;

impl U8CStr {
  pub fn decode(&self) -> Option<String> {
    decode_u8(self.as_slice())
  }
}

impl<const CAP: usize> StaticU8CStr<CAP> {
  pub fn encode(data: &str) -> Option<Self> {
    let encoded = encode_u8(data)?;
    if encoded.len() > CAP {
      return None;
    }
    Some(Self::from_slice(&encoded))
  }
  pub fn encode_truncate(data: &str) -> Option<Self> {
    let encoded = encode_u8(data)?;
    let len = core::cmp::min(encoded.len(), CAP);
    Some(Self::from_slice(&encoded[..len]))
  }
}

impl U8CString {
  pub fn encode(data: &str) -> Option<Self> {
    encode_u8(data).map(Into::into)
  }
}

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
impl<const CAPACITY: usize> std::io::Write for StaticU8CStr<CAPACITY> {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    if buf.is_empty() {
      return Ok(0);
    }
    let len = self.len_usize();
    let mut slice = &mut self.0[len..CAPACITY-1];
    let len = slice.write(buf)?;
    Ok(len)
  }

  fn flush(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

#[cfg(not(feature = "no_std"))]
impl std::io::Write for U8CString {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    let len = self.len_usize();
    if buf.is_empty() {
      return Ok(0);
    }
    self.0.truncate(len);
    let len = self.0.write(buf);
    if !matches!(self.0.last(), Some(&0)) {
      self.0.push(0);
    }
    let cap = self.0.capacity();
    self.0.resize(cap, 0);
    len
  }

  fn flush(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

impl<'a> From<&'a U8CStr> for &'a core::ffi::CStr {
  fn from(value: &'a U8CStr) -> Self {
    unsafe { std::ffi::CStr::from_bytes_until_nul(value.as_slice_with_nul()).unwrap_unchecked() }
  }
}
impl From<&core::ffi::CStr> for &U8CStr {
  fn from(value: &core::ffi::CStr) -> Self {
    unsafe { std::mem::transmute(value.to_bytes_with_nul()) }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Write;
  #[test]
  fn test_comparison() {
    let buf1 = *b"123\0";
    let cstr1: &U8CStr = (&buf1).try_into().unwrap();
    let buf2 = *b"123\0";
    let cstr2: &U8CStr = (&buf2).try_into().unwrap();
    assert_eq!(cstr1, cstr2);
    assert_eq!(cstr2, cstr1);
    let buf1 = *b"1234\0";
    let cstr1: &U8CStr = (&buf1).try_into().unwrap();
    let buf2 = *b"123\0";
    let cstr2: &U8CStr = (&buf2).try_into().unwrap();
    assert_ne!(cstr1, cstr2);
    assert_ne!(cstr2, cstr1);
    let buf1 = *b"123\04\0";
    let cstr1: &U8CStr = (&buf1).try_into().unwrap();
    let buf2 = *b"123\0";
    let cstr2: &U8CStr = (&buf2).try_into().unwrap();
    assert_eq!(cstr1, cstr2);
    assert_eq!(cstr2, cstr1);
  }
  #[test]
  fn test_comparison_static() {
    let buf1 = *b"123\0";
    let cstr1: StaticU8CStr<4> = (&buf1).try_into().unwrap();
    let buf2 = *b"123\0";
    let cstr2: &U8CStr = (&buf2).try_into().unwrap();
    assert_eq!(&*cstr1, cstr2);
    assert_eq!(cstr2, &*cstr1);
    let buf1 = *b"1234\0";
    let cstr1: StaticU8CStr<5> = (&buf1).try_into().unwrap();
    let buf2 = *b"123\0";
    let cstr2: StaticU8CStr<4> = (&buf2).try_into().unwrap();
    assert_ne!(cstr1, cstr2);
    assert_ne!(cstr2, cstr1);
    let buf1 = *b"123\04\0";
    let cstr1: StaticU8CStr<6> = (&buf1).try_into().unwrap();
    let buf2 = *b"123\0";
    let cstr2: StaticU8CStr<4> = (&buf2).try_into().unwrap();
    assert_eq!(cstr1, cstr2);
    assert_eq!(cstr2, cstr1);
  }
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
    assert_eq!(cstr.as_slice(), b"");
    assert_eq!(cstr.as_slice_with_nul(), b"\0");
    assert_eq!(cstr.as_slice_full(), b"\0");
    assert_eq!(buf, *b"456\0");
  }
  #[test]
  fn test_writes1() {
    let mut buf = *b"123\0";
    let mut cstr: &mut U8CStr = (&mut buf).try_into().unwrap();
    write!(cstr, "45").unwrap();
    assert_eq!(cstr.as_slice(), b"");
    assert_eq!(cstr.as_slice_with_nul(), b"\0");
    assert_eq!(cstr.as_slice_full(), b"\0\0");
    assert_eq!(buf, *b"45\0\0");
  }
  #[test]
  fn test_writes_mid_nul() {
    let mut buf = *b"123\0456\0";
    let mut cstr: &mut U8CStr = (&mut buf).try_into().unwrap();
    write!(cstr, "45").unwrap();
    assert_eq!(cstr.as_slice(), b"");
    assert_eq!(cstr.as_slice_with_nul(), b"\0");
    assert_eq!(cstr.as_slice_full(), b"\0\0456\0");
    assert_eq!(buf, *b"45\0\0456\0");
  }
  #[test]
  fn test_writes_over_mid_nul() {
    let mut buf = *b"123\0456\0";
    let mut cstr: &mut U8CStr = (&mut buf).try_into().unwrap();
    write!(cstr, "4567").unwrap();
    assert_eq!(cstr.as_slice(), b"");
    assert_eq!(cstr.as_slice_with_nul(), b"\0");
    assert_eq!(cstr.as_slice_full(), b"\056\0");
    assert_eq!(buf, *b"4567\056\0");
  }
  #[test]
  fn test_writes_continue() {
    let mut buf = *b"123456789\0";
    let mut cstr: &mut U8CStr = (&mut buf).try_into().unwrap();
    write!(cstr, "abc").unwrap();
    write!(cstr, "def").unwrap();
    assert_eq!(cstr.as_slice(), b"");
    assert_eq!(cstr.as_slice_with_nul(), b"\0");
    assert_eq!(cstr.as_slice_full(), b"\089\0");
    assert_eq!(buf, *b"abcdef\089\0");
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
  #[test]
  fn test_index() {
    let buf = b"abc\0";
    let cstr: StaticU8CStr<4> = buf.try_into().unwrap();
    let cstr: &CStr = cstr.as_ref();
    assert_eq!(cstr[0], b'a');
    assert_eq!(cstr[1], b'b');
    assert_eq!(cstr[2], b'c');
    assert_eq!(cstr[3], b'\0');
  }
  
  #[test]
  fn test_index_range() {
    let buf = b"abc\0";
    let cstr: StaticU8CStr<4> = buf.try_into().unwrap();
    let cstr: &CStr = cstr.as_ref();
    let abc: &U8CStr = b"abc\0".try_into().unwrap();
    let tmp = &cstr[0..]; 
    assert_eq!(tmp.len_with_nul_usize(), 4);
    assert_eq!(tmp.capacity_usize(), 4);
    assert_eq!(tmp.len_usize(), 3);
    assert_eq!(tmp, abc);
    let abc: &U8CStr = b"bc\0".try_into().unwrap();
    let tmp = &cstr[1..]; 
    assert_eq!(tmp.len_with_nul_usize(), 3);
    assert_eq!(tmp.capacity_usize(), 3);
    assert_eq!(tmp.len_usize(), 2);
    assert_eq!(tmp, abc);
    let abc: &U8CStr = b"c\0".try_into().unwrap();
    let tmp = &cstr[2..]; 
    assert_eq!(tmp.len_with_nul_usize(), 2);
    assert_eq!(tmp.capacity_usize(), 2);
    assert_eq!(tmp.len_usize(), 1);
    assert_eq!(tmp, abc);
    let abc: &U8CStr = b"\0".try_into().unwrap();
    let tmp = &cstr[3..]; 
    assert_eq!(tmp.len_with_nul_usize(), 1);
    assert_eq!(tmp.capacity_usize(), 1);
    assert_eq!(tmp.len_usize(), 0);
    assert_eq!(tmp, abc);
  }
  
  #[test]
  fn test_static_index() {
    let buf = b"abc\0";
    let cstr: StaticU8CStr<4> = buf.try_into().unwrap();
    assert_eq!(cstr[0], b'a');
    assert_eq!(cstr[1], b'b');
    assert_eq!(cstr[2], b'c');
    assert_eq!(cstr[3], b'\0');
  }
  
  #[test]
  fn test_static_index_range() {
    let buf = b"abc\0";
    let cstr: StaticU8CStr<4> = buf.try_into().unwrap();
    let abc: &U8CStr = b"abc\0".try_into().unwrap();
    let tmp = &cstr[0..]; 
    assert_eq!(tmp.len_with_nul_usize(), 4);
    assert_eq!(tmp.capacity_usize(), 4);
    assert_eq!(tmp.len_usize(), 3);
    assert_eq!(tmp, abc);
    let abc: &U8CStr = b"bc\0".try_into().unwrap();
    let tmp = &cstr[1..]; 
    assert_eq!(tmp.len_with_nul_usize(), 3);
    assert_eq!(tmp.capacity_usize(), 3);
    assert_eq!(tmp.len_usize(), 2);
    assert_eq!(tmp, abc);
    let abc: &U8CStr = b"c\0".try_into().unwrap();
    let tmp = &cstr[2..]; 
    assert_eq!(tmp.len_with_nul_usize(), 2);
    assert_eq!(tmp.capacity_usize(), 2);
    assert_eq!(tmp.len_usize(), 1);
    assert_eq!(tmp, abc);
    let abc: &U8CStr = b"\0".try_into().unwrap();
    let tmp = &cstr[3..]; 
    assert_eq!(tmp.len_with_nul_usize(), 1);
    assert_eq!(tmp.capacity_usize(), 1);
    assert_eq!(tmp.len_usize(), 0);
    assert_eq!(tmp, abc);
  }
  #[test]
  fn test_owning_str() {
    extern "C" {
      fn free(ptr: *mut std::ffi::c_void);
      fn calloc(num: usize, size: usize) -> *mut std::ffi::c_void;
    }
    let cnt = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let cnt_c = std::sync::Arc::clone(&cnt);
    let deleter = |ptr: *mut u8| {
      unsafe{free(ptr.cast())};
      cnt_c.store(true, core::sync::atomic::Ordering::Relaxed);
    };
    let data = unsafe { calloc(1, 5) }.cast();
    let tmp = unsafe { std::slice::from_raw_parts_mut(data, 5) };
    tmp.copy_from_slice(b"abcd\0");
    let string = unsafe { U8OwningCStr::from_ptr_safe_deleter(data, deleter) };
    assert_eq!(format!("{}", string.display()), "abcd");
    drop(string);
    assert!(cnt.load(core::sync::atomic::Ordering::Relaxed))
  }
  #[test]
  fn test_owning_str_free() {
    extern "C" {
      fn calloc(num: usize, size: usize) -> *mut std::ffi::c_void;
    }
    let data = unsafe { calloc(1, 5) }.cast();
    let tmp = unsafe { std::slice::from_raw_parts_mut(data, 5) };
    tmp.copy_from_slice(b"abcd\0");
    let string = unsafe { U8OwningCStr::new(data) };
    assert_eq!(format!("{}", string.display()), "abcd");
  }
}
