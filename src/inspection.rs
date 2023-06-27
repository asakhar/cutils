#[macro_export]
macro_rules! current_function {
  () => {{
    fn f() {}
    fn type_name_of<T>(_: T) -> &'static str {
      std::any::type_name::<T>()
    }
    let name = type_name_of(f);
    name.strip_suffix("::f").unwrap()
  }};
}

pub trait InitZeroed {
  unsafe fn init_zeroed() -> Self
  where
    Self: Sized,
  {
    std::mem::zeroed()
  }
}

impl<T> InitZeroed for T {}

pub trait GetPtrExt {
  fn get_const_ptr(&self) -> *const Self {
    self as *const _
  }
  fn get_mut_ptr(&mut self) -> *mut Self {
    self as *mut _
  }
}

impl<T> GetPtrExt for T {}

pub trait CastToConstVoidPtrExt {
  fn cast_to_pcvoid(self) -> *const core::ffi::c_void;
}

impl<T> CastToConstVoidPtrExt for *const T {
  fn cast_to_pcvoid(self) -> *const core::ffi::c_void {
    self as *const _
  }
}

pub trait CastToMutVoidPtrExt {
  fn cast_to_pvoid(self) -> *mut core::ffi::c_void;
}

impl<T> CastToMutVoidPtrExt for *mut T {
  fn cast_to_pvoid(self) -> *mut core::ffi::c_void {
    self as *mut _
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn function_name() {
    let current_function: &str = current_function!();
    assert_eq!(current_function, "cutils::inspection::tests::function_name");
  }
}
