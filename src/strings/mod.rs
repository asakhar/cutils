mod common;
mod cstr;
mod str_macro;
mod string_macro;
mod static_str_macro;
mod u16cstr;
mod u32cstr;
mod writes;
use std::{ffi::{OsString, OsStr}, os::windows::prelude::{OsStringExt, OsStrExt}};

pub use cstr::*;
pub use u16cstr::*;
pub use u32cstr::*;
pub use writes::*;

pub trait CStrCharType {
  type Char;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrError {
  NulNotFound,
}

/// Alias for [`U16CStr`] or [`U32CStr`] depending on platform. Intended to match typical C
/// `wchar_t` size on platform.
#[cfg(not(windows))]
pub type WideCStr = U32CStr;

/// Alias for [`U16CStr`] or [`U32CStr`] depending on platform. Intended to match typical C
/// `wchar_t` size on platform.
#[cfg(windows)]
pub type WideCStr = U16CStr;

/// Alias for [`U16CString`] or [`U32CString`] depending on platform. Intended to match typical C
/// `wchar_t` size on platform.
#[cfg(not(windows))]
pub type WideCString = U32CString;

/// Alias for [`U16CString`] or [`U32CString`] depending on platform. Intended to match typical C
/// `wchar_t` size on platform.
#[cfg(windows)]
pub type WideCString = U16CString;

/// Alias for [`StaticU16CStr`] or [`StaticU32CStr`] depending on platform. Intended to match typical C
/// `wchar_t` size on platform.
#[cfg(not(windows))]
pub type StaticWideCStr<const CAPACITY: usize> = StaticU32CStr<CAPACITY>;

/// Alias for [`StaticU16CStr`] or [`StaticU32CStr`] depending on platform. Intended to match typical C
/// `wchar_t` size on platform.
#[cfg(windows)]
pub type StaticWideCStr<const CAPACITY: usize> = StaticU16CStr<CAPACITY>;

impl WideCStr {
  pub fn to_os_string(&self) -> OsString {
    OsString::from_wide(self.as_slice())
  }
}

impl WideCString {
  pub fn to_os_string(&self) -> OsString {
    OsString::from_wide(self.as_slice())
  }
}

impl From<OsString> for WideCString {
  fn from(value: OsString) -> Self {
    let inner: Vec<u16> = value.encode_wide().collect();
    Self::from(inner)
  }
}

impl From<&OsStr> for WideCString {
  fn from(value: &OsStr) -> Self {
    let inner: Vec<u16> = value.encode_wide().collect();
    Self::from(inner)
  }
}

impl core::fmt::Display for StrError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("Nul-terminator was not found in source")
  }
}

#[cfg(feature = "widestring")]
mod widestr_convs;

#[doc(hidden)]
pub mod internals;
