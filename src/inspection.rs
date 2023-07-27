#[macro_export]
macro_rules! current_function {
  () => {{
    fn f() {}
    let name = $crate::type_name_of(f);
    name.strip_suffix("::f").unwrap()
  }};
}

pub trait GetPtrExt {
  fn get_const_ptr(&self) -> *const Self {
    self as *const _
  }
  fn get_mut_ptr(&mut self) -> *mut Self {
    self as *mut _
  }
}

impl<T> GetPtrExt for T {}

#[macro_export]
macro_rules! csizeof {
  ($name:path) => {
    core::mem::size_of::<$name>() as _
  };
  (=$val:expr) => {
    core::mem::size_of_val(&$val) as _
  };
  ($name:path; $type:ty) => {
    core::mem::size_of::<$name>() as $type
  };
  (=$val:expr; $type:ty) => {
    core::mem::size_of_val(&$val) as $type
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
    let r: u32 = csizeof!(u64);
    assert_eq!(r, 8);
    let r: u32 = csizeof!(u32);
    assert_eq!(r, 4);
    let r: usize = csizeof!(u16);
    assert_eq!(r, 2);
    let r: i32 = csizeof!(u8);
    assert_eq!(r, 1);
    let r: u32 = csizeof!(usize);
    #[cfg(target_pointer_width = "64")]
    assert_eq!(r, 8);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(r, 4);
    #[cfg(target_pointer_width = "16")]
    assert_eq!(r, 2);
    #[repr(C)]
    struct A {
      a: u8,
      b: u64,
      c: [u8; 5],
    }
    let r: usize = csizeof!(A);
    assert_eq!(r, 24);
    let val = A {
      a: 0,
      b: 0,
      c: [0; 5],
    };
    let r: i32 = csizeof!(=val);
    assert_eq!(r, 24);
    let r: u8 = csizeof!(=val.a as u64+val.b);
    assert_eq!(r, 8);
  }
}
