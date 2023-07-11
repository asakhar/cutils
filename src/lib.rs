#![cfg_attr(feature = "no_std", no_std)]
pub mod arrays;
pub mod deferred;
pub mod definitions;
pub mod ignore;
pub mod inspection;
pub mod strings;
pub mod files;
pub use cutils_macro::*;

#[cfg(feature = "get-last-error")]
use get_last_error::Win32Error;
#[cfg(feature = "winapi")]
use winapi::{
  shared::{guiddef::GUID, minwindef::DWORD, ntdef::HANDLE, winerror::ERROR_SUCCESS},
  um::{cfgmgr32::CONFIGRET, errhandlingapi::SetLastError, handleapi::INVALID_HANDLE_VALUE},
};

#[cfg(feature = "get-last-error")]
pub type Win32Result<T> = Result<T, Win32Error>;

#[cfg(feature = "get-last-error")]
pub fn set_last_error(error: Win32Error) {
  unsafe {
    SetLastError(error.code());
  }
}

#[cfg(feature = "get-last-error")]
pub trait Win32ErrorToResultExt {
  fn to_result(self) -> Win32Result<()>;
}

#[cfg(feature = "get-last-error")]
impl Win32ErrorToResultExt for Win32Error {
  fn to_result(self) -> Win32Result<()> {
    if self.code() == ERROR_SUCCESS {
      Ok(())
    } else {
      Err(self)
    }
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
pub trait Win32ErrorFromCrExt {
  fn from_cr(ret: CONFIGRET, default: DWORD) -> Win32Error;
}

#[cfg(feature = "winapi")]
impl Win32ErrorFromCrExt for Win32Error {
  fn from_cr(ret: CONFIGRET, default: DWORD) -> Win32Error {
    let err = unsafe { CM_MapCrToWin32Err(ret, default) };
    Win32Error::new(err)
  }
}

#[cfg(feature = "winapi")]
extern "C" {
  fn CM_MapCrToWin32Err(CmReturnCode: CONFIGRET, DefaultErr: DWORD) -> DWORD;
}
