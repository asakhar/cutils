use super::common::{
  common_cstr_impls, common_cstring_impls, common_str_writes_impl, common_string_writes_impl,
};
common_cstr_impls!(U32CStr, u32, U32CString, DisplayU32CStr);
common_cstring_impls!(U32CString, u32, U32CStr);

common_str_writes_impl!(U32CStr, length_as_u32);
common_string_writes_impl!(U32CString, length_as_u32);

#[cfg(not(feature = "no_std"))]
impl super::writes::io::Write32 for &mut U32CStr {
  fn write32(&mut self, buf: &[u32]) -> std::io::Result<usize> {
    let writable = self.capacity_usize() - 1;
    let written = std::cmp::min(buf.len(), writable);
    unsafe { self.as_mut_slice_full()[0..written].copy_from_slice(&buf[0..written]) };
    unsafe { self.as_mut_slice_full()[written] = 0 };
    let new_self = &mut unsafe { self.as_mut_slice_full() }[written..];

    *self = unsafe {
      U32CStr::from_mut_slice_unchecked(std::slice::from_raw_parts_mut(
        new_self.as_mut_ptr(),
        new_self.len(),
      ))
    };
    Ok(written)
  }

  fn flush32(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

#[cfg(not(feature = "no_std"))]
impl super::writes::io::Write32 for U32CString {
  fn write32(&mut self, buf: &[u32]) -> std::io::Result<usize> {
    let (inner, len) = self.inner();
    inner.resize(*len + buf.len() + 1, 0);
    inner[*len..*len + buf.len()].copy_from_slice(buf);
    inner[*len + buf.len()] = 0;
    inner.resize(inner.capacity(), 0);
    self.refresh();
    Ok(buf.len())
  }

  fn flush32(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

impl super::writes::fmt::Write32 for &mut U32CStr {
  fn write32_str(&mut self, buf: &U32CStr) -> core::fmt::Result {
    let space = unsafe { self.as_mut_slice_full() };
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
    let (inner, len) = self.inner();
    inner.resize(*len + buf.len_with_nul_usize(), 0);
    inner[*len..*len + buf.len_with_nul_usize()].copy_from_slice(buf.as_slice_with_nul());
    self.refresh();
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
    str.write32_fmt(format_args!("abc{}", 1)).unwrap();
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
    str.write32_fmt(format_args!("abc{}", 1)).unwrap();
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
