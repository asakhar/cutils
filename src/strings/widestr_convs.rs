macro_rules! common_conv_cstr_impl {
  ($src:ty, $dst:ty) => {
    impl<'a> From<&'a $src> for &'a $dst {
      fn from(value: &'a $src) -> Self {
        unsafe { <$dst>::from_slice_unchecked(value.as_slice_with_nul()) }
      }
    }
    impl<'a> From<&'a mut $src> for &'a mut $dst {
      fn from(value: &'a mut $src) -> Self {
        unsafe { <$dst>::from_slice_unchecked_mut(value.as_mut_slice_with_nul()) }
      }
    }
    impl From<&$dst> for &$src {
      fn from(value: &$dst) -> Self {
        unsafe { std::mem::transmute(value.as_slice()) }
      }
    }
    impl From<&mut $dst> for &mut $src {
      fn from(value: &mut $dst) -> Self {
        unsafe { std::mem::transmute(value.as_mut_slice()) }
      }
    }
  };
}
common_conv_cstr_impl!(super::U16CStr, widestring::U16CStr);
common_conv_cstr_impl!(super::U32CStr, widestring::U32CStr);
macro_rules! common_conv_cstring_impl {
  ($src:ty, $dst:ty) => {
    impl From<$src> for $dst {
      fn from(value: $src) -> Self {
        Self::from_vec_truncate(value)
      }
    }
    impl From<$dst> for $src {
      fn from(value: $dst) -> Self {
        value.into_vec_with_nul().into()
      }
    }
  };
}
common_conv_cstring_impl!(super::U16CString, widestring::U16CString);
common_conv_cstring_impl!(super::U32CString, widestring::U32CString);
