mod common;
mod cstr;
mod u16cstr;
mod u32cstr;
mod str_macro;
mod string_macro;
mod writes;
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

impl core::fmt::Display for StrError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("Nul-terminator was not found in source")
  }
}

#[cfg(feature = "widestring")]
mod widestr_convs;

#[doc(hidden)]
pub mod internals;