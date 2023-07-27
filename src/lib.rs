#![cfg_attr(feature = "no_std", no_std)]
pub mod arrays;
pub mod deferred;
pub mod definitions;
pub mod files;
pub mod ignore;
pub mod inspection;
pub mod strings;
pub use cutils_macro::*;
pub mod errors;

pub fn type_name_of<T>(_: T) -> &'static str {
  core::any::type_name::<T>()
}

#[cfg(feature = "winapi")]
use winapi::{
  shared::{guiddef::GUID, minwindef::DWORD, ntdef::HANDLE, winerror::ERROR_SUCCESS},
  um::{cfgmgr32::CONFIGRET, errhandlingapi::SetLastError, handleapi::INVALID_HANDLE_VALUE},
};

#[cfg(feature = "winapi")]
pub fn set_last_error(error: std::io::Error) {
  unsafe {
    SetLastError(
      error
        .raw_os_error()
        .map(|e| e as u32)
        .unwrap_or(ERROR_SUCCESS),
    );
  }
}

#[cfg(feature = "winapi")]
pub fn guid_eq(lhs: GUID, rhs: GUID) -> bool {
  lhs.Data1 == rhs.Data1
    && lhs.Data2 == rhs.Data2
    && lhs.Data3 == rhs.Data3
    && lhs.Data4 == rhs.Data4
}

#[cfg(feature = "winapi")]
pub fn code_to_result(code: DWORD) -> std::io::Result<()> {
  if code == ERROR_SUCCESS {
    Ok(())
  } else {
    Err(std::io::Error::from_raw_os_error(code as i32))
  }
}

#[cfg(feature = "winapi")]
pub fn check_handle(handle: HANDLE) -> bool {
  !handle.is_null() && handle != INVALID_HANDLE_VALUE
}

#[cfg(feature = "winapi")]
pub trait ErrorFromCrExt {
  fn from_cr(ret: CONFIGRET, default: DWORD) -> Self;
}

#[cfg(feature = "winapi")]
impl ErrorFromCrExt for std::io::Error {
  fn from_cr(ret: CONFIGRET, default: DWORD) -> Self {
    let err = unsafe { CM_MapCrToWin32Err(ret, default) };
    std::io::Error::from_raw_os_error(err as i32)
  }
}

#[cfg(feature = "winapi")]
extern "system" {
  fn CM_MapCrToWin32Err(CmReturnCode: CONFIGRET, DefaultErr: DWORD) -> DWORD;
}
