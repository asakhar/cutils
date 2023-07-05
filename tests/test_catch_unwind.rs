use cutils::unwind_catch;

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