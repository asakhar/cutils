use cutils::strings::*;
use cutils::*;
fn as_u32(v: &[u8]) -> Vec<u32> {
  let mut r = Vec::new();
  for i in v {
    r.push(*i as u32);
  }
  r
}

#[test]
fn test_display() {
  let cstr = u32cstr!("123");
  assert_eq!("123", format!("{}", cstr.display()));
}
#[test]
fn test_u32cstr_macro() {
  let string = u32cstr!("123");
  assert_eq!(string.as_slice_full(), as_u32(b"123\0"));
  assert_eq!(string.as_slice(), as_u32(b"123"));
}
#[test]
fn test_u32cstr_macro_rlit() {
  let string = u32cstr!(r#""123""#);
  assert_eq!(string.as_slice_full(), as_u32(b"\"123\"\0"));
  assert_eq!(string.as_slice(), as_u32(b"\"123\""));
}
#[test]
fn test_u32cstr_macro_with_internal_nul() {
  let string = u32cstr!("123\0");
  assert_eq!(string.as_slice_full(), as_u32(b"123\0\0"));
  assert_eq!(string.as_slice(), as_u32(b"123"));
}
#[test]
fn test_u32cstr_macro_const_str() {
  const TEST: &str = "123";
  const STRING: &U32CStr = u32cstr!(TEST);
  assert_eq!(STRING.as_slice_full(), as_u32(b"123\0"));
  assert_eq!(STRING.as_slice(), as_u32(b"123"));
}
#[test]
fn test_u32cstring_macro() {
  let string = u32cstring!("123");
  assert_eq!(string.as_slice_with_nul(), as_u32(b"123\0"));
  assert_eq!(string.as_slice(), as_u32(b"123"));
}

#[test]
fn test_u32cstring_macro_fmt() {
  let string = u32cstring!("123 {}", 456);
  assert_eq!(string.as_slice_with_nul(), as_u32(b"123 456\0"));
  assert_eq!(string.as_slice(), as_u32(b"123 456"));
}
