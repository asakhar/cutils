use std::path::{Path, PathBuf};

use get_last_error::Win32Error;
use winapi::shared::minwindef::MAX_PATH;
use winapi::shared::{minwindef::DWORD, ntdef::HANDLE};
use winapi::um::fileapi::{CreateFileA, CreateFileW};
use winapi::um::fileapi::{
  CREATE_ALWAYS, CREATE_NEW, OPEN_ALWAYS, OPEN_EXISTING, TRUNCATE_EXISTING,
};
use winapi::um::handleapi::CloseHandle;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::winnt::FILE_ATTRIBUTE_NORMAL;
use winapi::um::winnt::{
  FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_ALL, GENERIC_EXECUTE, GENERIC_READ,
  GENERIC_WRITE,
};

use crate::strings::{CStr, WideCStr};
use crate::{check_handle, Win32Result};
use winapi::um::sysinfoapi::GetWindowsDirectoryW;
use winapi::um::winnt::WCHAR;

pub trait FileName {
  fn open<'a, 'b>(
    &self,
    options: WindowsFileOpenOptions<'a, 'b>,
    disposition: DWORD,
  ) -> Win32Result<WindowsFile>;
}

impl<T: AsRef<WideCStr>> FileName for T {
  fn open<'a, 'b>(
    &self,
    options: WindowsFileOpenOptions<'a, 'b>,
    disposition: DWORD,
  ) -> Win32Result<WindowsFile> {
    let handle = unsafe {
      CreateFileW(
        self.as_ref().as_ptr(),
        options.desired_access,
        options.share_mode,
        options
          .security_attributes
          .map(|s| s as *const _ as *mut _)
          .unwrap_or(std::ptr::null_mut()),
        disposition,
        options.flags_and_attributes,
        options
          .template_file
          .map(|t| t.handle)
          .unwrap_or(std::ptr::null_mut()),
      )
    };
    if !check_handle(handle) {
      return Err(Win32Error::get_last_error());
    }
    Ok(WindowsFile { handle })
  }
}

impl FileName for CStr {
  fn open<'a, 'b>(
    &self,
    options: WindowsFileOpenOptions<'a, 'b>,
    disposition: DWORD,
  ) -> Win32Result<WindowsFile> {
    let handle = unsafe {
      CreateFileA(
        self.as_ptr() as *const _,
        options.desired_access,
        options.share_mode,
        options
          .security_attributes
          .map(|s| s as *const _ as *mut _)
          .unwrap_or(std::ptr::null_mut()),
        disposition,
        options.flags_and_attributes,
        options
          .template_file
          .map(|t| t.handle)
          .unwrap_or(std::ptr::null_mut()),
      )
    };
    if !check_handle(handle) {
      return Err(Win32Error::get_last_error());
    }
    Ok(WindowsFile { handle })
  }
}

#[derive(Clone, Copy)]
pub struct WindowsFileOpenOptions<'a, 'b> {
  desired_access: DWORD,
  share_mode: DWORD,
  security_attributes: Option<&'a SECURITY_ATTRIBUTES>,
  flags_and_attributes: DWORD,
  template_file: Option<&'b WindowsFile>,
}

impl<'a, 'b> Default for WindowsFileOpenOptions<'a, 'b> {
  fn default() -> Self {
    Self {
      desired_access: GENERIC_READ,
      share_mode: 0,
      security_attributes: None,
      flags_and_attributes: FILE_ATTRIBUTE_NORMAL,
      template_file: None,
    }
  }
}

impl<'a, 'b> WindowsFileOpenOptions<'a, 'b> {
  pub fn read(mut self, generic_read: bool) -> Self {
    if generic_read {
      self.desired_access |= GENERIC_READ;
    } else {
      self.desired_access &= !GENERIC_READ;
    }
    self
  }
  pub fn write(mut self, generic_write: bool) -> Self {
    if generic_write {
      self.desired_access |= GENERIC_WRITE;
    } else {
      self.desired_access &= !GENERIC_WRITE;
    }
    self
  }
  pub fn execute(mut self, generic_execute: bool) -> Self {
    if generic_execute {
      self.desired_access |= GENERIC_EXECUTE;
    } else {
      self.desired_access &= !GENERIC_EXECUTE;
    }
    self
  }
  pub fn reset_access(mut self, all_or_none: bool) -> Self {
    self.desired_access = if all_or_none { GENERIC_ALL } else { 0 };
    self
  }
  pub fn share_read(mut self, share_read: bool) -> Self {
    if share_read {
      self.share_mode |= FILE_SHARE_READ;
    } else {
      self.share_mode &= !FILE_SHARE_READ;
    }
    self
  }
  pub fn share_write(mut self, share_write: bool) -> Self {
    if share_write {
      self.share_mode |= FILE_SHARE_WRITE;
    } else {
      self.share_mode &= !FILE_SHARE_WRITE;
    }
    self
  }
  pub fn share_delete(mut self, share_delete: bool) -> Self {
    if share_delete {
      self.share_mode |= FILE_SHARE_DELETE;
    } else {
      self.share_mode &= !FILE_SHARE_DELETE;
    }
    self
  }
  pub fn reset_sharing(mut self, all_or_none: bool) -> Self {
    self.share_mode = if all_or_none {
      FILE_SHARE_WRITE | FILE_SHARE_READ | FILE_SHARE_DELETE
    } else {
      0
    };
    self
  }
  pub fn reset_security_attributes<'n>(
    self,
    security_attributes: Option<&'n SECURITY_ATTRIBUTES>,
  ) -> WindowsFileOpenOptions<'n, 'b> {
    WindowsFileOpenOptions {
      desired_access: self.desired_access,
      share_mode: self.share_mode,
      security_attributes,
      flags_and_attributes: self.flags_and_attributes,
      template_file: self.template_file,
    }
  }
  pub fn reset_flags_and_attributes(mut self, flags_and_attributes: DWORD) -> Self {
    self.flags_and_attributes = flags_and_attributes;
    self
  }
  pub fn reset_template_file<'n>(
    self,
    template: Option<&'n WindowsFile>,
  ) -> WindowsFileOpenOptions<'a, 'n> {
    WindowsFileOpenOptions {
      desired_access: self.desired_access,
      share_mode: self.share_mode,
      security_attributes: self.security_attributes,
      flags_and_attributes: self.flags_and_attributes,
      template_file: template,
    }
  }
  pub fn create_always(self, name: &impl FileName) -> Win32Result<WindowsFile> {
    name.open(self, CREATE_ALWAYS)
  }
  pub fn create_new(self, name: &impl FileName) -> Win32Result<WindowsFile> {
    name.open(self, CREATE_NEW)
  }
  pub fn open_always(self, name: &impl FileName) -> Win32Result<WindowsFile> {
    name.open(self, OPEN_ALWAYS)
  }
  pub fn open_existing(self, name: &impl FileName) -> Win32Result<WindowsFile> {
    name.open(self, OPEN_EXISTING)
  }
  pub fn truncate_existing(self, name: &impl FileName) -> Win32Result<WindowsFile> {
    name.open(self, TRUNCATE_EXISTING)
  }
  pub fn open_custom_disposition(
    self,
    name: &impl FileName,
    disposition: DWORD,
  ) -> Win32Result<WindowsFile> {
    name.open(self, disposition)
  }
}

pub struct WindowsFile {
  handle: HANDLE,
}

impl Drop for WindowsFile {
  fn drop(&mut self) {
    unsafe {
      CloseHandle(self.handle);
    }
  }
}

impl WindowsFile {
  pub fn options() -> WindowsFileOpenOptions<'static, 'static> {
    WindowsFileOpenOptions::default()
  }
}

pub fn get_windows_dir_path() -> Win32Result<PathBuf> {
  let mut windows_directory = [0 as WCHAR; MAX_PATH];
  let result = unsafe { GetWindowsDirectoryW(windows_directory.as_mut_ptr(), MAX_PATH as u32) };
  if result == 0 {
    return Err(Win32Error::get_last_error());
  }
  let windows_dir = unsafe { WideCStr::from_ptr(windows_directory.as_ptr()) };
  let windows_dir = windows_dir.to_os_string();
  Ok(Path::new(&windows_dir).to_owned())
}
