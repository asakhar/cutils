#[macro_export]
macro_rules! u8cstring {
  ($s:expr) => {{
    let tmp = $crate::u8cstr!($s);
    tmp.to_owned()
  }};
  ($s:literal, $($args:expr),+) => {{
    let mut tmp = $crate::strings::U8CString::new();
    use std::io::Write;
    drop(write!(tmp, $s, $($args),+));
    tmp
  }};
}

#[macro_export]
macro_rules! u16cstring {
  ($s:expr) => {{
    let tmp = $crate::u16cstr!($s);
    tmp.to_owned()
  }};
  ($s:literal, $($args:expr),+) => {{
    let mut tmp = $crate::strings::U16CString::new();
    use std::io::Write;
    drop(write!(tmp, $s, $($args),+));
    tmp
  }};
}

#[macro_export]
macro_rules! u32cstring {
  ($s:expr) => {{
    let tmp = $crate::u32cstr!($s);
    tmp.to_owned()
  }};
  ($s:literal, $($args:expr),+) => {{
    let mut tmp = $crate::strings::U32CString::new();
    use std::io::Write;
    drop(write!(tmp, $s, $($args),+));
    tmp
  }};
}

#[macro_export]
macro_rules! cstring {
  ($s:tt $(,$args:expr)*) => {
    $crate::u8cstring!($s $(,$args)*)  
  };
}

#[cfg(windows)]
mod windows {
  #[macro_export]
  macro_rules! widecstring {
    ($s:tt $(,$args:expr)*) => {
      $crate::u16cstring!($s $(,$args)*)
    };
  }
}

#[cfg(not(windows))]
mod not_windows {
  #[macro_export]
  macro_rules! widecstring {
    ($s:tt $(,$args:expr)*) => {
      $crate::u32cstring!($s $(,$args)*)
    };
  }
}