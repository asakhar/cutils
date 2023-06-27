#[macro_export]
macro_rules! u16_array {
  ($str:literal; $size:expr; $default:expr) => {
    {
      const BYTES: &[u8] = $str.as_bytes();
      const ARRAY: [u16; $size] = {
        $crate::strings::internals::panic_on_invalid_utf8(BYTES);
        let mut src = BYTES;
        let mut buf = [0 as u16; $size];
        let mut i = 0;
        while let Some((ch, rest)) = unsafe { $crate::strings::internals::next_code_point(src) } {
          src = rest;
          buf[i] = ch as u16;
          i += 1;
        }
        buf
      };
      ARRAY
    }
  };
  ($str:literal; $size:expr) => {
    u16_array![$str; $size; 0]
  }
}
#[macro_export]
macro_rules! u32_array {
  ($str:literal; $size:expr; $default:expr) => {
    {
      const BYTES: &[u8] = $str.as_bytes();
      const ARRAY: [u32; $size] = {
        $crate::strings::internals::panic_on_invalid_utf8(BYTES);
        let mut src = BYTES;
        let mut buf = [0 as u32; $size];
        let mut i = 0;
        while let Some((ch, rest)) = unsafe { $crate::strings::internals::next_code_point(src) } {
          src = rest;
          buf[i] = ch as u32;
          i += 1;
        }
        buf
      };
      ARRAY
    }
  };
  ($str:literal; $size:expr) => {
    u32_array![$str; $size; 0]
  }
}
#[macro_export]
macro_rules! wide_array {
  ($str:literal; $size:expr; $default:expr) => {
    {
      const BYTES: &[u8] = $str.as_bytes();
      const ARRAY: [$crate::definitions::WChar; $size] = {
        $crate::strings::internals::panic_on_invalid_utf8(BYTES);
        let mut src = BYTES;
        let mut buf = [0 as $crate::definitions::WChar; $size];
        let mut i = 0;
        while let Some((ch, rest)) = unsafe { $crate::strings::internals::next_code_point(src) } {
          src = rest;
          buf[i] = ch as $crate::definitions::WChar;
          i += 1;
        }
        buf
      };
      ARRAY
    }
  };
  ($str:literal; $size:expr) => {
    wide_array![$str; $size; 0]
  }
}

#[cfg(test)]
mod tests {
  use crate::definitions::WChar;

  #[test]
  fn creates_wide() {
    const WIDE_ARRAY: [WChar; 10] = wide_array!["123"; 10];
    assert_eq!(
      WIDE_ARRAY,
      [
        b'1' as WChar,
        b'2' as WChar,
        b'3' as WChar,
        0,
        0,
        0,
        0,
        0,
        0,
        0
      ]
    );
  }
  #[test]
  fn creates_u16() {
    const U16_ARRAY: [u16; 10] = u16_array!["123"; 10];
    assert_eq!(
      U16_ARRAY,
      [
        b'1' as u16,
        b'2' as u16,
        b'3' as u16,
        0,
        0,
        0,
        0,
        0,
        0,
        0
      ]
    );
  }
  #[test]
  fn creates_u32() {
    const U32_ARRAY: [u32; 10] = u32_array!["123"; 10];
    assert_eq!(
      U32_ARRAY,
      [
        b'1' as u32,
        b'2' as u32,
        b'3' as u32,
        0,
        0,
        0,
        0,
        0,
        0,
        0
      ]
    );
  }
}
