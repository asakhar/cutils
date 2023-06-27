#[cfg(feature = "macro")]
mod macro_tests {
  use cutils::strings::*;
  use cutils::*;

  #[test]
  fn test_cstr_macro() {
    let string = cstr!("123");
    assert_eq!(string.as_slice_full(), b"123\0");
    assert_eq!(string.as_slice(), b"123");
  }
  #[test]
  fn test_cstr_macro_rlit() {
    let string = cstr!(r#""123""#);
    assert_eq!(string.as_slice_full(), b"\"123\"\0");
    assert_eq!(string.as_slice(), b"\"123\"");
  }


  #[test]
  fn test_cstr_macro_with_internal_nul() {
    let string = cstr!("123\0");
    assert_eq!(string.as_slice_full(), b"123\0\0");
    assert_eq!(string.as_slice(), b"123");
  }
  #[test]
  fn test_cstr_macro_const_str() {
    const TEST: &str = "123";
    const STRING: &U8CStr = cstr!(TEST);
    assert_eq!(STRING.as_slice_full(), b"123\0");
    assert_eq!(STRING.as_slice(), b"123");
  }
  #[test]
  fn test_cstring_macro() {
    let string = cstring!("123");
    assert_eq!(string.as_slice_with_nul(), b"123\0");
    assert_eq!(string.as_slice(), b"123");
  }

  #[test]
  fn test_cstring_macro_fmt() {
    let string = cstring!("123 {} {}", 456, "abc");
    assert_eq!(string.as_slice_with_nul(), b"123 456 abc\0");
    assert_eq!(string.as_slice(), b"123 456 abc");
  }
}
