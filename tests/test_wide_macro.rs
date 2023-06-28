mod macro_tests {
  use cutils::*;
  use cutils::strings::*;
  type CharType = <WideCStr as CStrCharType>::Char;
  fn as_wide(v: &[u8]) -> Vec<CharType> {
    let mut r = Vec::new();
    for i in v {
      r.push(*i as CharType);
    }
    r
  }
  #[test]
  fn test_widecstr_macro() {
    let string = widecstr!("123");
    assert_eq!(string.as_slice_full(), as_wide(b"123\0"));
    assert_eq!(string.as_slice(), as_wide(b"123"));
  }
  #[test]
  fn test_widecstr_macro_rlit() {
    let string = widecstr!(r#""123""#);
    assert_eq!(string.as_slice_full(), as_wide(b"\"123\"\0"));
    assert_eq!(string.as_slice(), as_wide(b"\"123\""));
  }
  #[test]
  fn test_widecstr_macro_with_internal_nul() {
    let string = widecstr!("123\0");
    assert_eq!(string.as_slice_full(), as_wide(b"123\0\0"));
    assert_eq!(string.as_slice(), as_wide(b"123"));
  }
  #[test]
  fn test_widecstr_macro_const_str() {
    const TEST: &str = "123";
    const STRING: &WideCStr = widecstr!(TEST);
    assert_eq!(STRING.as_slice_full(), as_wide(b"123\0"));
    assert_eq!(STRING.as_slice(), as_wide(b"123"));
  }
  #[test]
  fn test_widecstring_macro() {
    let string = widecstring!("123");
    assert_eq!(string.as_slice_with_nul(), as_wide(b"123\0"));
    assert_eq!(string.as_slice(), as_wide(b"123"));
  }
  
  #[test]
  fn test_widecstring_macro_fmt() {
    let string = widecstring!("123 {}", 456);
    assert_eq!(string.as_slice_with_nul(), as_wide(b"123 456\0"));
    assert_eq!(string.as_slice(), as_wide(b"123 456"));
  }
}
