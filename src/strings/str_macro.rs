macro_rules! impl_macro {
  ($name:ident, $type:ty, $item:ty, $fn:ident) => {
    #[macro_export]
    macro_rules! $name {
      ($s:tt) => {{
        const BYTES: &[u8] = $s.as_bytes();
        const LEN: usize = unsafe {
          $crate::strings::internals::$fn(BYTES)
        };
        const BUF: [$item; LEN + 1] = {
          $crate::strings::internals::panic_on_invalid_utf8(BYTES);
          let mut src = BYTES;
          let mut buf = [0 as $item; LEN + 1];
          let mut i = 0;
          while let Some((ch, rest)) = unsafe { $crate::strings::internals::next_code_point(src) } {
            src = rest;
            buf[i] = ch as $item;
            i += 1;
          }
          buf
        };
        unsafe { $crate::strings::$type::from_slice_unchecked(&BUF) }
      }};
    }
  };
}

impl_macro!(u8cstr, U8CStr, u8, length_as_u8_or_panic);
impl_macro!(u16cstr, U16CStr, u16, length_as_u16_or_panic);
impl_macro!(u32cstr, U32CStr, u32, length_as_u32_or_panic);

#[macro_export]
macro_rules! cstr {
  ($s:tt) => {
    $crate::u8cstr!($s)
  };
}

#[cfg(windows)]
mod windows {
  #[macro_export]
  macro_rules! widecstr {
    ($s:tt) => {
      $crate::u16cstr!($s)
    };
  }
}

#[cfg(not(windows))]
mod not_windows {
  #[macro_export]
  macro_rules! widecstr {
    ($s:tt) => {
      $crate::u32cstr!($s)
    };
  }
}
