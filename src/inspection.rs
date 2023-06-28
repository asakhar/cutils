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

#[macro_export]
macro_rules! csizeof {
  ($name:path) => {
    core::mem::size_of::<$name>() as u32
  };
  (=$val:expr) => {
    core::mem::size_of_val(&$val) as u32
  };
}

#[cfg(test)]
mod tests {
  #[test]
  fn function_name() {
    let current_function: &str = current_function!();
    assert_eq!(current_function, "cutils::inspection::tests::function_name");
  }
  #[test]
  fn csizeof_test() {
    assert_eq!(csizeof!(u64), 8);
    assert_eq!(csizeof!(u32), 4);
    assert_eq!(csizeof!(u16), 2);
    assert_eq!(csizeof!(u8), 1);
    #[cfg(target_pointer_width = "64")]
    assert_eq!(csizeof!(usize), 8);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(csizeof!(usize), 4);
    #[cfg(target_pointer_width = "16")]
    assert_eq!(csizeof!(usize), 2);
    #[repr(C)]
    struct A {
      a: u8,
      b: u64,
      c: [u8; 5],
    }
    assert_eq!(csizeof!(A), 24);
    let val = A {
      a: 0,
      b: 0,
      c: [0; 5],
    };
    assert_eq!(csizeof!(=val), 24);
    assert_eq!(csizeof!(=val.a as u64+val.b), 8);
  }
}
