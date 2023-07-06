#[macro_export]
macro_rules! static_u8cstr {
  ($s:literal; $size:expr $(,$args:expr)*) => {{
    let mut tmp = $crate::strings::StaticU8CStr::<$size>::zeroed();
    use std::io::Write;
    drop(write!(tmp, $s $(, $args)*));
    tmp
  }};
}

#[macro_export]
macro_rules! static_u16cstr {
  ($s:literal; $size:expr $(,$args:expr)*) => {{
    let mut tmp = $crate::strings::StaticU16CStr::<$size>::zeroed();
    use std::io::Write;
    drop(write!(tmp, $s $(,$args)*));
    tmp
  }};
}

#[macro_export]
macro_rules! static_u32cstr {
  ($s:literal; $size:expr $(,$args:expr)*) => {{
    let mut tmp = $crate::strings::StaticU32CStr::<$size>::zeroed();
    use std::io::Write;
    drop(write!(tmp, $s $(,$args)*));
    tmp
  }};
}

#[macro_export]
macro_rules! static_cstr {
  ($s:literal; $size:expr $(,$args:expr)*) => {
    $crate::static_u8cstr!($s; $size $(,$args)*)  
  };
}

#[cfg(windows)]
mod windows {
  #[macro_export]
  macro_rules! static_widecstr {
    ($s:literal; $size:expr $(,$args:expr)*) => {
      $crate::static_u16cstr!($s; $size $(,$args)*)
    };
  }
}

#[cfg(not(windows))]
mod not_windows {
  #[macro_export]
  macro_rules! static_widecstr {
    ($s:literal; $size:expr $(,$args:expr)*) => {
      $crate::static_u32cstr!($s; $size $(,$args)*)
    };
  }
}