use super::{
  common::{
    common_cstr_impls, common_cstring_impls, common_staticcstr_impls, common_staticstr_writes_impl,
    common_str_writes_impl, common_string_writes_impl,
  },
  internals::{decode_u32, encode_u32},
};
common_cstr_impls!(
  U32CStr,
  u32,
  U32CString,
  DisplayU32CStr,
  U32CStrIter,
  StaticU32CStr
);
common_staticcstr_impls!(
  StaticU32CStr,
  u32,
  U32CString,
  U32CStr,
  DisplayU32CStr,
  StaticU32CStrIntoIter,
  super::internals::encode_u32
);
common_cstring_impls!(U32CString, u32, U32CStr, DisplayU32CStr, U32CStringIter, super::internals::encode_u32);

common_str_writes_impl!(U32CStr, length_as_u32);
common_string_writes_impl!(U32CString, length_as_u32);
common_staticstr_writes_impl!(StaticU32CStr<CAPACITY>, length_as_u32);

impl U32CStr {
  pub fn decode(&self) -> Option<String> {
    decode_u32(self.as_slice())
  }
}

impl<const CAP: usize> StaticU32CStr<CAP> {
  pub fn encode(data: &str) -> Option<Self> {
    let encoded = encode_u32(data)?;
    if encoded.len() > CAP {
      return None;
    }
    Some(Self::from_slice(&encoded))
  }
  pub fn encode_truncate(data: &str) -> Self {
    let encoded = encode_u32(data).unwrap();
    let len = core::cmp::min(encoded.len(), CAP);
    Self::from_slice(&encoded[..len])
  }
}

impl U32CString {
  pub fn encode(data: &str) -> Self {
    encode_u32(data).unwrap().into()
  }
}

#[cfg(not(feature = "no_std"))]
impl super::writes::io::Write32 for &mut U32CStr {
  fn write32(&mut self, buf: &[u32]) -> std::io::Result<usize> {
    let writable = self.capacity_usize();
    let written = std::cmp::min(buf.len(), writable);
    self.as_mut_slice_full()[0..written].copy_from_slice(&buf[0..written]);
    let new_self = &mut self.0[written..];

    *self = unsafe { U32CStr::from_mut_ptr_unchecked(new_self.as_mut_ptr(), new_self.len()) };
    Ok(written)
  }

  fn flush32(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

#[cfg(not(feature = "no_std"))]
impl super::writes::io::Write32 for U32CString {
  fn write32(&mut self, buf: &[u32]) -> std::io::Result<usize> {
    let len = self.len_usize();
    if buf.is_empty() {
      return Ok(0);
    }
    self.0.truncate(len);
    let len = self.0.write32(buf);
    if !matches!(self.0.last(), Some(&0)) {
      self.0.push(0);
    }
    let cap = self.0.capacity();
    self.0.resize(cap, 0);
    len
  }

  fn flush32(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

impl super::writes::fmt::Write32 for &mut U32CStr {
  fn write32_str(&mut self, buf: &U32CStr) -> core::fmt::Result {
    let space = self.as_mut_slice_full();
    if space.len() < buf.len_with_nul_usize() {
      return Err(core::fmt::Error);
    }
    space[0..buf.len_with_nul_usize()].copy_from_slice(buf.as_slice_with_nul());
    *self = unsafe { core::mem::transmute(&mut space[buf.len_usize()..]) };
    Ok(())
  }
}

impl super::writes::fmt::Write32 for U32CString {
  fn write32_str(&mut self, buf: &U32CStr) -> core::fmt::Result {
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
  use crate::strings::io::Write32;

  use super::{U32CStr, U32CString};

  #[test]
  fn writes32_cstr() {
    let mut buf = [0; 5];
    let mut str = unsafe { U32CStr::from_mut_slice_unchecked(&mut buf) };
    str.write32(&[1, 2, 3, 4]).unwrap();
    assert_eq!(str.as_slice_full(), &[0]);
    assert_eq!(buf, [1, 2, 3, 4, 0]);
  }
  #[test]
  fn writes32_cstr_twice() {
    let mut buf = [0; 5];
    let mut str = unsafe { U32CStr::from_mut_slice_unchecked(&mut buf) };
    str.write32(&[1, 2]).unwrap();
    assert_eq!(str.as_slice_full(), &[0, 0, 0]);
    str.write32(&[3, 4]).unwrap();
    assert_eq!(str.as_slice_full(), &[0]);
    assert_eq!(buf, [1, 2, 3, 4, 0]);
  }
  #[test]
  fn writes32_cstr_fmt() {
    let mut buf = [0; 5];
    let mut str = unsafe { U32CStr::from_mut_slice_unchecked(&mut buf) };
    use std::fmt::Write;
    str.write_fmt(format_args!("abc{}", 1)).unwrap();
    assert_eq!(str.as_slice_full(), &[0]);
    assert_eq!(buf, [b'a' as u32, b'b' as u32, b'c' as u32, b'1' as u32, 0]);
  }

  #[test]
  fn writes32_cstring() {
    let mut str = U32CString::new();
    str.write32_all(&[1, 2, 3, 4]).unwrap();
    assert_eq!(str.as_slice_with_nul(), &[1, 2, 3, 4, 0]);
  }

  #[test]
  fn writes32_cstring_fmt() {
    let mut str = U32CString::new();
    use std::io::Write;
    str.write_fmt(format_args!("abc{}", 1)).unwrap();
    assert_eq!(
      str.as_slice_with_nul(),
      &[b'a' as u32, b'b' as u32, b'c' as u32, b'1' as u32, 0]
    );
  }

  #[test]
  fn writes32_cstring_twice() {
    let mut str = U32CString::new();
    str.write32(&[1, 2]).unwrap();
    assert_eq!(str.as_slice_with_nul(), &[1, 2, 0]);
    str.write32(&[3, 4]).unwrap();
    assert_eq!(str.as_slice_with_nul(), &[1, 2, 3, 4, 0]);
  }
}
