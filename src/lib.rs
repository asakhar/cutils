use winapi::shared::guiddef::GUID;

pub mod arrays;
pub mod deferred;
pub mod definitions;
pub mod ignore;
pub mod inspection;
pub mod strings;

use get_last_error::Win32Error;
use winapi::{
  shared::{
    minwindef::DWORD,
    ntdef::{HANDLE, PVOID},
    winerror::{ERROR_GEN_FAILURE, ERROR_SUCCESS},
  },
  um::{
    cfgmgr32::{CONFIGRET, PCVOID},
    errhandlingapi::SetLastError,
    handleapi::INVALID_HANDLE_VALUE,
  }
};

pub type Win32Result<T> = Result<T, Win32Error>;

pub fn set_last_error(error: Win32Error) {
  unsafe {
    SetLastError(error.code());
  }
}

pub trait Win32ErrorToResultExt {
  fn to_result(self) -> Win32Result<()>;
}

impl Win32ErrorToResultExt for Win32Error {
  fn to_result(self) -> Win32Result<()> {
    if self.code() == ERROR_SUCCESS {
      Ok(())
    } else {
      Err(self)
    }
  }
}


pub fn guid_eq(lhs: GUID, rhs: GUID) -> bool {
  lhs.Data1 == rhs.Data1
    && lhs.Data2 == rhs.Data2
    && lhs.Data3 == rhs.Data3
    && lhs.Data4 == rhs.Data4
}

#[cfg(windows)]
pub fn code_to_result(code: DWORD) -> Win32Result<()> {
  if code == ERROR_SUCCESS {
    Ok(())
  } else {
    Win32Error::new(code).to_result()
  }
}

pub fn check_handle(handle: HANDLE) -> bool {
  !handle.is_null() && handle != INVALID_HANDLE_VALUE
}

pub trait GetPvoidExt {
  fn get_pvoid(&self) -> PVOID {
    self as *const Self as PCVOID as PVOID
  }
}

impl<T> GetPvoidExt for T {}

pub trait Win32ErrorFromCrExt {
  fn from_cr(ret: CONFIGRET) -> Win32Error;
}

impl Win32ErrorFromCrExt for Win32Error {
  fn from_cr(ret: CONFIGRET) -> Win32Error {
    let err = unsafe { CM_MapCrToWin32Err(ret, ERROR_GEN_FAILURE) };
    Win32Error::new(err)
  }
}

extern "C" {
  fn CM_MapCrToWin32Err(CmReturnCode: CONFIGRET, DefaultErr: DWORD) -> DWORD;
}