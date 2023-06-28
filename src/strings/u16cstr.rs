use super::common::{
  common_cstr_impls, common_cstring_impls, common_str_writes_impl, common_string_writes_impl,
};
common_cstr_impls!(U16CStr, u16, U16CString, DisplayU16CStr);
common_cstring_impls!(U16CString, u16, U16CStr);

common_str_writes_impl!(U16CStr, length_as_u16);
common_string_writes_impl!(U16CString, length_as_u16);

#[cfg(not(feature = "no_std"))]
impl super::writes::io::Write16 for &mut U16CStr {
  fn write16(&mut self, buf: &[u16]) -> std::io::Result<usize> {
    let writable = self.capacity_usize() - 1;
    let written = std::cmp::min(buf.len(), writable);
    unsafe { self.as_mut_slice_full()[0..written].copy_from_slice(&buf[0..written]) };
    unsafe { self.as_mut_slice_full()[written] = 0 };
    let new_self = &mut unsafe { self.as_mut_slice_full() }[written..];

    *self = unsafe {
      U16CStr::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(
        new_self.as_mut_ptr(),
        new_self.len(),
      ))
    };
    Ok(written)
  }

  fn flush16(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

#[cfg(not(feature = "no_std"))]
impl super::writes::io::Write16 for U16CString {
  fn write16(&mut self, buf: &[u16]) -> std::io::Result<usize> {
    let (inner, len) = self.inner();
    inner.resize(*len + buf.len() + 1, 0);
    inner[*len..*len + buf.len()].copy_from_slice(buf);
    inner[*len + buf.len()] = 0;
    inner.resize(inner.capacity(), 0);
    self.refresh();
    Ok(buf.len())
  }

  fn flush16(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

impl super::writes::fmt::Write16 for &mut U16CStr {
  fn write16_str(&mut self, buf: &U16CStr) -> core::fmt::Result {
    let space = unsafe { self.as_mut_slice_full() };
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
    let (inner, len) = self.inner();
    inner.resize(*len + buf.len_with_nul_usize(), 0);
    inner[*len..*len + buf.len_with_nul_usize()].copy_from_slice(buf.as_slice_with_nul());
    self.refresh();
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::{strings::io::Write16};

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
    str.write16_fmt(format_args!("abc{}", 1)).unwrap();
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
    str.write16_fmt(format_args!("abc{}", 1)).unwrap();
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
