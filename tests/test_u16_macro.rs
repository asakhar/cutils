
use cutils::strings::*;
use cutils::*;
fn as_u16(v: &[u8]) -> Vec<u16> {
  let mut r = Vec::new();
  for i in v {
    r.push(*i as u16);
  }
  r
}
#[test]
fn test_display() {
  let cstr = u16cstr!("123");
  assert_eq!("123", format!("{}", cstr.display()));
}
#[test]
fn test_u16cstr_macro() {
  let string = u16cstr!("123");
  assert_eq!(string.as_slice_full(), as_u16(b"123\0"));
  assert_eq!(string.as_slice(), as_u16(b"123"));
}
#[test]
fn test_u16cstr_macro_rlit() {
  let string = u16cstr!(r#""123""#);
  assert_eq!(string.as_slice_full(), as_u16(b"\"123\"\0"));
  assert_eq!(string.as_slice(), as_u16(b"\"123\""));
}
#[test]
fn test_u16cstr_macro_with_internal_nul() {
  let string = u16cstr!("123\0");
  assert_eq!(string.as_slice_full(), as_u16(b"123\0\0"));
  assert_eq!(string.as_slice(), as_u16(b"123"));
}
#[test]
fn test_u16cstr_macro_unicode() {
  let string = u16cstr!("123Ā");
  let encoded = widestring::u16cstr!("123Ā");
  assert_eq!(string.as_slice_full(), encoded.as_slice_with_nul());
  assert_eq!(string.as_slice(), encoded.as_slice());
}
#[test]
fn test_u16cstr_macro_const_str() {
  const TEST: &str = "123";
  const STRING: &U16CStr = u16cstr!(TEST);
  assert_eq!(STRING.as_slice_full(), as_u16(b"123\0"));
  assert_eq!(STRING.as_slice(), as_u16(b"123"));
}
#[test]
fn test_u16cstring_macro() {
  let string = u16cstring!("123");
  assert_eq!(string.as_slice_with_nul(), as_u16(b"123\0"));
  assert_eq!(string.as_slice(), as_u16(b"123"));
}

#[test]
fn test_u16cstring_macro_fmt() {
  let string = u16cstring!("123 {}", 456);
  assert_eq!(string.as_slice_with_nul(), as_u16(b"123 456\0"));
  assert_eq!(string.as_slice(), as_u16(b"123 456"));
}
