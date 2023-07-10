use super::{common::{
  common_cstr_impls, common_cstring_impls, common_staticcstr_impls, common_staticstr_writes_impl,
  common_str_writes_impl, common_string_writes_impl,
}, internals::{decode_u16, encode_u16}};
common_cstr_impls!(U16CStr, u16, U16CString, DisplayU16CStr, U16CStrIter, StaticU16CStr);
common_staticcstr_impls!(StaticU16CStr, u16, U16CString, U16CStr, DisplayU16CStr, StaticU16CStrIntoIter);
common_cstring_impls!(U16CString, u16, U16CStr, DisplayU16CStr, U16CStringIter);

common_str_writes_impl!(U16CStr, length_as_u16);
common_string_writes_impl!(U16CString, length_as_u16);
common_staticstr_writes_impl!(StaticU16CStr<CAPACITY>, length_as_u16);

impl U16CStr {
  pub fn decode(&self) -> Option<String> {
    decode_u16(self.as_slice())
  }
}

impl<const CAP: usize> StaticU16CStr<CAP> {
  pub fn encode(data: &str) -> Option<Self> {
    let encoded = encode_u16(data)?;
    if encoded.len() > CAP {
      return None;
    }
    Some(Self::from_slice(&encoded))
  }
}

impl U16CString {
  pub fn encode(data: &str) -> Option<Self> {
    encode_u16(data).map(Into::into)
  }
}

#[cfg(not(feature = "no_std"))]
impl super::writes::io::Write16 for &mut U16CStr {
  fn write16(&mut self, buf: &[u16]) -> std::io::Result<usize> {
    let writable = self.capacity_usize();
    let written = std::cmp::min(buf.len(), writable);
    self.as_mut_slice_full()[0..written].copy_from_slice(&buf[0..written]);
    let new_self = &mut  self.0[written..];

    *self = unsafe { core::mem::transmute(new_self) };
    Ok(written)
  }

  fn flush16(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

#[cfg(not(feature = "no_std"))]
impl super::writes::io::Write16 for U16CString {
  fn write16(&mut self, buf: &[u16]) -> std::io::Result<usize> {
    let len = self.len_usize();
    if buf.is_empty() {
      return Ok(0);
    }
    self.0.truncate(len);
    let len = self.0.write16(buf);
    if !matches!(self.0.last(), Some(&0)) {
      self.0.push(0);
    }
    let cap = self.0.capacity();
    self.0.resize(cap, 0);
    len
  }

  fn flush16(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

impl super::writes::fmt::Write16 for &mut U16CStr {
  fn write16_str(&mut self, buf: &U16CStr) -> core::fmt::Result {
    let space = self.as_mut_slice_full() ;
    if space.len() < buf.len_with_nul_usize() {
      return Err(core::fmt::Error);
    }
    space[0..buf.len_with_nul_usize()].copy_from_slice(buf.as_slice_with_nul());
    *self = unsafe { core::mem::transmute(&mut space[buf.len_usize()..]) };
    Ok(())
  }
}

impl super::writes::fmt::Write16 for U16CString {
  fn write16_str(&mut self, buf: &U16CStr) -> core::fmt::Result {
    let len = self.len_usize();
    self.0.resize(len + buf.len_with_nul_usize(), 0);
    self.0[len..len + buf.len_with_nul_usize()].copy_from_slice(buf.as_slice_with_nul());
    let cap = self.0.capacity();
    self.0.resize(cap, 0);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::strings::io::Write16;

  use super::{U16CStr, U16CString};

  #[test]
  fn writes16_cstr() {
    let mut buf = [0; 5];
    let mut str = unsafe { U16CStr::from_mut_slice_unchecked(&mut buf) };
    str.write16_all(&[1, 2, 3, 4]).unwrap();
    assert_eq!(str.as_slice_full(), &[0]);
    assert_eq!(buf, [1, 2, 3, 4, 0]);
  }
  #[test]
  fn writes16_cstr_twice() {
    let mut buf = [0; 5];
    let mut str = unsafe { U16CStr::from_mut_slice_unchecked(&mut buf) };
    str.write16_all(&[1, 2]).unwrap();
    assert_eq!(str.as_slice_full(), &[0, 0, 0]);
    str.write16_all(&[3, 4]).unwrap();
    assert_eq!(str.as_slice_full(), &[0]);
    assert_eq!(buf, [1, 2, 3, 4, 0]);
  }
  #[test]
  fn writes16_cstr_fmt() {
    let mut buf = [0; 5];
    let mut str = unsafe { U16CStr::from_mut_slice_unchecked(&mut buf) };
    use std::fmt::Write;
    str.write_fmt(format_args!("abc{}", 1)).unwrap();
    assert_eq!(str.as_slice_full(), &[0]);
    assert_eq!(buf, [b'a' as u16, b'b' as u16, b'c' as u16, b'1' as u16, 0]);
  }

  #[test]
  fn writes16_cstring() {
    let mut str = U16CString::new();
    str.write16_all(&[1, 2, 3, 4]).unwrap();
    assert_eq!(str.as_slice_with_nul(), &[1, 2, 3, 4, 0]);
  }

  #[test]
  fn writes16_cstring_fmt() {
    let mut str = U16CString::new();
    use std::io::Write;
    str.write_fmt(format_args!("abc{}", 1)).unwrap();
    assert_eq!(
      str.as_slice_with_nul(),
      &[b'a' as u16, b'b' as u16, b'c' as u16, b'1' as u16, 0]
    );
  }

  #[test]
  fn writes16_cstring_twice() {
    let mut str = U16CString::new();
    str.write16(&[1, 2]).unwrap();
    assert_eq!(str.as_slice_with_nul(), &[1, 2, 0]);
    str.write16(&[3, 4]).unwrap();
    assert_eq!(str.as_slice_with_nul(), &[1, 2, 3, 4, 0]);
  }
}
