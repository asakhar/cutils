use cutils::{unwind_catch, unwind_handle};

#[test]
fn test_catches() {
  #[unwind_catch(-1)]
  unsafe extern "C" fn export_abi() -> i32 {
    panic!("catch me");
  }
  assert_eq!(unsafe { export_abi() }, -1);
}

#[test]
fn test_catches_with_args() {
  #[unwind_catch(-1)]
  unsafe extern "C" fn export_abi(switch: bool) -> i32 {
    if switch {
      panic!("catch me");
    } else {
      2
    }
  }
  assert_eq!(unsafe { export_abi(true) }, -1);
  assert_eq!(unsafe { export_abi(false) }, 2);
}

#[test]
fn test_catches_with_generics() {
  #[unwind_catch(Default::default())]
  unsafe extern "C" fn export_abi<T: Default + From<i8>>(switch: bool) -> T {
    if switch {
      panic!("catch me");
    } else {
      2.into()
    }
  }
  assert_eq!(unsafe { export_abi::<i16>(true) }, Default::default());
  assert_eq!(unsafe { export_abi::<i32>(false) }, 2);
}

#[test]
fn test_handles() {
  fn handler(err: Box<dyn std::any::Any + Send>) -> i32 {
    let Ok(err) = err.downcast::<std::io::Error>() else {
      return -1;
    };
    err.raw_os_error().unwrap_or(-2)
  }
  #[unwind_handle(handler)]
  unsafe extern "C" fn export_abi() -> i32 {
    panic!("catch me");
  }
  assert_eq!(unsafe { export_abi() }, -1);
}
#[test]
fn test_handles_io_error() {
  fn handler(err: Box<dyn std::any::Any + Send>) -> i32 {
    if let Ok(msg) = err.downcast::<&str>() {
      if *msg == "msg" {
        return 5;
      }
      return -2;
    };
    -1
  }
  #[unwind_handle(handler)]
  unsafe extern "C" fn export_abi() -> i32 {
    panic!("msg")
  }
  assert_eq!(unsafe { export_abi() }, 5);
}