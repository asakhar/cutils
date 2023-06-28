macro_rules! common_cstr_impls {
  ($name:ident, $type:ty, $into:ty, $display:ident) => {
    #[repr(transparent)]
    pub struct $name([$type]);
    impl $crate::strings::CStrCharType for $name {
      type Char = $type;
    }
    pub struct $display<'a>(&'a [$type]);
    impl<'a> core::fmt::Display for $display<'a> {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        use core::fmt::Write;
        for ch in self.0 {
          f.write_char(char::from_u32(*ch as u32).ok_or(core::fmt::Error)?)?;
        }
        Ok(())
      }
    }
    impl $name {
      // Waiting for https://github.com/rust-lang/rust/issues/8995 to be stabilized
      // pub type Char = $type;
      // For now working around using trait
      pub const fn len(&self) -> u32 {
        self.len_usize() as u32
      }
      pub const fn is_empty(&self) -> bool {
        self.len_usize() == 0
      }
      pub const fn len_usize(&self) -> usize {
        let mut items = &self.0;
        let mut i = 0;
        while let Some(item) = items.first() {
          if *item == 0 {
            return i;
          }
          items = match items {
            [_, rest @ ..] => rest,
            [] => &[],
          };
          i += 1
        }
        unreachable!()
      }
      pub const fn len_with_nul(&self) -> u32 {
        self.len() + 1
      }
      pub const fn len_with_nul_usize(&self) -> usize {
        self.len_usize() + 1
      }
      pub const fn capacity(&self) -> u32 {
        self.capacity_usize() as u32
      }
      pub const fn capacity_usize(&self) -> usize {
        self.0.len()
      }
      pub const fn as_slice(&self) -> &[$type] {
        // Const implementation of: "&self.0[0..self.len_usize()]""
        unsafe { core::slice::from_raw_parts(self.0.as_ptr(), self.len_usize()) }
      }
      pub unsafe fn as_mut_slice(&mut self) -> &mut [$type] {
        let len = self.len_usize();
        &mut self.0[0..len]
      }
      pub const fn as_slice_with_nul(&self) -> &[$type] {
        // Const implementation of: "&self.0[0..self.len_with_nul_usize()]"
        unsafe { core::slice::from_raw_parts(self.0.as_ptr(), self.len_with_nul_usize()) }
      }
      pub unsafe fn as_mut_slice_with_nul(&mut self) -> &mut [$type] {
        let len = self.len_with_nul_usize();
        &mut self.0[0..len]
      }
      pub const fn as_slice_full(&self) -> &[$type] {
        &self.0
      }
      pub unsafe fn as_mut_slice_full(&mut self) -> &mut [$type] {
        &mut self.0
      }
      pub const fn as_ptr(&self) -> *const $type {
        self.0.as_ptr()
      }
      pub fn as_mut_ptr(&mut self) -> *mut $type {
        self.0.as_mut_ptr()
      }
      pub const unsafe fn as_mut_ptr_bypass(&self) -> *mut $type {
        self.0.as_ptr() as *mut _
      }
      pub const unsafe fn from_slice_unchecked(data: &[$type]) -> &Self {
        core::mem::transmute(data)
      }
      pub unsafe fn from_mut_slice_unchecked(data: &mut [$type]) -> &mut Self {
        core::mem::transmute(data)
      }
      pub const unsafe fn from_ptr<'a>(data: *const $type) -> &'a Self {
        let mut inf_buf = core::slice::from_raw_parts(data, usize::MAX);
        // Const implementation of: "let len = inf_buf.iter().take_while(|c| **c != 0).count();"
        // -----
        let mut len = 0;
        while let Some(item) = inf_buf.first() {
          if *item == 0 {
            break;
          }
          inf_buf = match inf_buf {
            [_, rest @ ..] => rest,
            [] => unreachable!(),
          };
          len += 1;
        }
        // -----
        let buf = core::slice::from_raw_parts(data, len + 1);
        core::mem::transmute(buf)
      }
      pub unsafe fn from_mut_ptr<'a>(data: *mut $type) -> &'a mut Self {
        let inf_buf = core::slice::from_raw_parts(data, usize::MAX);
        let len = inf_buf.iter().take_while(|c| **c != 0).count();
        let buf = core::slice::from_raw_parts_mut(data, len + 1);
        core::mem::transmute(buf)
      }
      pub const unsafe fn from_ptr_unchecked<'a>(data: *const $type, capacity: usize) -> &'a Self {
        let buf = core::slice::from_raw_parts(data, capacity);
        core::mem::transmute(buf)
      }
      pub unsafe fn from_mut_ptr_unchecked<'a>(data: *mut $type, capacity: usize) -> &'a mut Self {
        let buf = core::slice::from_raw_parts_mut(data, capacity);
        core::mem::transmute(buf)
      }
      pub const unsafe fn from_ptr_n<'a>(
        data: *const $type,
        max_len: usize,
      ) -> Result<&'a Self, $crate::strings::StrError> {
        let mut inf_buf = core::slice::from_raw_parts(data, max_len);
        // Const implementation of: "let len = inf_buf.iter().take_while(|c| **c != 0).count();"
        // -----
        let mut len = 0;
        while let Some(item) = inf_buf.first() {
          if *item == 0 {
            break;
          }
          len += 1;
          inf_buf = match inf_buf {
            [_, rest @ ..] => rest,
            [] => &[],
          };
        }
        // -----
        if len == max_len {
          Err($crate::strings::StrError::NulNotFound)
        } else {
          let buf = core::slice::from_raw_parts(data, len + 1);
          Ok(core::mem::transmute(buf))
        }
      }
      pub unsafe fn from_mut_ptr_n<'a>(
        data: *mut $type,
        max_len: usize,
      ) -> Result<&'a mut Self, $crate::strings::StrError> {
        let inf_buf = core::slice::from_raw_parts(data, max_len);
        let len = inf_buf.iter().take_while(|c| **c != 0).count();
        if len == max_len {
          Err($crate::strings::StrError::NulNotFound)
        } else {
          let buf = core::slice::from_raw_parts_mut(data, len + 1);
          Ok(core::mem::transmute(buf))
        }
      }
      pub fn display<'a>(&'a self) -> $display<'a> {
        $display(&self.0[0..self.len_usize()])
      }
    }
    impl TryFrom<&[$type]> for &$name {
      type Error = $crate::strings::StrError;
      fn try_from(value: &[$type]) -> Result<Self, $crate::strings::StrError> {
        if value.iter().take_while(|c| **c != 0).count() == value.len() {
          Err($crate::strings::StrError::NulNotFound)
        } else {
          Ok(unsafe { core::mem::transmute(value) })
        }
      }
    }
    impl TryFrom<&mut [$type]> for &mut $name {
      type Error = $crate::strings::StrError;
      fn try_from(value: &mut [$type]) -> Result<Self, $crate::strings::StrError> {
        if value.iter().take_while(|c| **c != 0).count() == value.len() {
          Err($crate::strings::StrError::NulNotFound)
        } else {
          Ok(unsafe { core::mem::transmute(value) })
        }
      }
    }
    impl<const N: usize> TryFrom<&[$type; N]> for &$name {
      type Error = $crate::strings::StrError;
      fn try_from(value: &[$type; N]) -> Result<Self, $crate::strings::StrError> {
        if value.iter().take_while(|c| **c != 0).count() == value.len() {
          Err($crate::strings::StrError::NulNotFound)
        } else {
          Ok(unsafe { core::mem::transmute(value as &[$type]) })
        }
      }
    }
    impl<const N: usize> TryFrom<&mut [$type; N]> for &mut $name {
      type Error = $crate::strings::StrError;
      fn try_from(value: &mut [$type; N]) -> Result<Self, $crate::strings::StrError> {
        if value.iter().take_while(|c| **c != 0).count() == value.len() {
          Err($crate::strings::StrError::NulNotFound)
        } else {
          Ok(unsafe { core::mem::transmute(value as &mut [$type]) })
        }
      }
    }
    impl From<&$name> for $into {
      fn from(value: &$name) -> Self {
        Self::from(&value.0)
      }
    }
    impl ToOwned for $name {
      type Owned = $into;
      fn to_owned(&self) -> Self::Owned {
        self.into()
      }
    }
    impl AsRef<$name> for &$name {
      fn as_ref(&self) -> &$name {
        self
      }
    }
    impl AsMut<$name> for &mut $name {
      fn as_mut(&mut self) -> &mut $name {
        self
      }
    }
  };
}

macro_rules! common_cstring_impls {
  ($name:ident, $type:ty, $asref:ty, $display:ident) => {
    pub struct $name(core::cell::UnsafeCell<(Vec<$type>, usize)>);
    unsafe impl Send for $name {}
    unsafe impl Sync for $name {}
    impl $crate::strings::CStrCharType for $name {
      type Char = $type;
    }
    impl $name {
      pub fn new() -> Self {
        let mut buf = vec![0 as $type];
        buf.resize(buf.capacity(), 0);
        Self(core::cell::UnsafeCell::new((buf, 0)))
      }
      pub fn with_capacity(cap: usize) -> Self {
        let mut buf = vec![0 as $type; cap + 1];
        buf.resize(buf.capacity(), 0);
        Self(core::cell::UnsafeCell::new((buf, 0)))
      }
      fn inner(&self) -> &mut (Vec<$type>, usize) {
        unsafe { &mut *self.0.get() }
      }
      fn refresh(&self) -> usize {
        let inner = self.inner();
        let cap = inner.0.capacity();
        inner.0.resize(cap, 0);
        let len = inner.0.iter().take_while(|c| **c != 0).count();
        inner.1 = len;
        len
      }
      pub fn reserve_usize(&mut self, additional: usize) {
        let inner = self.inner();
        let cap = inner.0.capacity();
        inner.0.resize(cap, 0);
        let len = inner.0.iter().take_while(|c| **c != 0).count();
        let new_len = len+additional as usize;
        inner.0.resize(new_len, 0);
        let cap = inner.0.capacity();
        inner.0.resize(cap, 0);
        inner.1 = new_len;
      }
      pub fn reserve(&mut self, additional: u32) {
        self.reserve_usize(additional as usize)
      }
      pub fn len(&self) -> u32 {
        self.len_usize() as u32
      }
      pub fn len_usize(&self) -> usize {
        self.refresh()
      }
      pub fn is_empty(&self) -> bool {
        self.len_usize() == 0
      }
      pub fn len_with_nul(&self) -> u32 {
        self.len() + 1
      }
      pub fn len_with_nul_usize(&self) -> usize {
        self.len_usize() + 1
      }
      pub fn len_hint(&self) -> u32 {
        self.len_hint_usize() as u32
      }
      pub fn len_hint_usize(&self) -> usize {
        self.inner().1
      }
      pub fn len_hint_with_nul(&self) -> u32 {
        self.len_hint() + 1
      }
      pub fn len_hint_with_nul_usize(&self) -> usize {
        self.len_hint_usize() + 1
      }
      pub fn capacity(&self) -> u32 {
        self.capacity_usize() as u32
      }
      pub fn capacity_usize(&self) -> usize {
        self.inner().0.len() - 1
      }
      pub fn as_slice(&self) -> &[$type] {
        let len = self.len_usize();
        &self.inner().0[0..len]
      }
      pub unsafe fn as_mut_slice(&mut self) -> &mut [$type] {
        let len = self.len_usize();
        &mut self.inner().0[0..len]
      }
      pub fn as_slice_with_nul(&self) -> &[$type] {
        let len = self.len_with_nul_usize();
        &self.inner().0[0..len]
      }
      pub unsafe fn as_mut_slice_with_nul(&mut self) -> &mut [$type] {
        let len = self.len_with_nul_usize();
        &mut self.inner().0[0..len]
      }
      pub fn as_slice_full(&self) -> &[$type] {
        &self.inner().0
      }
      pub unsafe fn as_mut_slice_full(&mut self) -> &mut [$type] {
        &mut self.inner().0
      }
      pub fn as_ptr(&self) -> *const $type {
        self.inner().0.as_ptr()
      }
      pub fn as_mut_ptr(&mut self) -> *mut $type {
        self.inner().0.as_mut_ptr()
      }
      pub unsafe fn from_ptr(data: *const $type) -> Self {
        let inf_buf = core::slice::from_raw_parts(data, usize::MAX);
        let len = inf_buf.iter().take_while(|c| **c != 0).count();
        let buf = core::slice::from_raw_parts(data, len + 1).to_vec();
        Self(core::cell::UnsafeCell::new((buf, len)))
      }
      pub unsafe fn from_ptr_unchecked(data: *const $type, len: usize, capacity: usize) -> Self {
        let buf = core::slice::from_raw_parts(data, capacity).to_vec();
        Self(core::cell::UnsafeCell::new((buf, len)))
      }
      pub unsafe fn from_ptr_unchecked_calc_len(data: *const $type, capacity: usize) -> Self {
        let buf = core::slice::from_raw_parts(data, capacity).to_vec();
        let len = buf.iter().take_while(|c| **c != 0).count();
        Self(core::cell::UnsafeCell::new((buf, len)))
      }
      pub unsafe fn from_ptr_n(
        data: *const $type,
        max_len: usize,
      ) -> Result<Self, $crate::strings::StrError> {
        let inf_buf = core::slice::from_raw_parts(data, max_len);
        let len = inf_buf.iter().take_while(|c| **c != 0).count();
        if len == max_len {
          Err($crate::strings::StrError::NulNotFound)
        } else {
          let buf = core::slice::from_raw_parts(data, len + 1).to_vec();
          Ok(Self(core::cell::UnsafeCell::new((buf, len))))
        }
      }

      pub unsafe fn from_ptr_truncate(data: *const $type, max_len: usize) -> Self {
        let inf_buf = core::slice::from_raw_parts(data, max_len);
        let len = inf_buf.iter().take_while(|c| **c != 0).count();
        let buf = if len == max_len {
          let mut buf = core::slice::from_raw_parts(data, len).to_vec();
          buf.push(0);
          buf
        } else {
          core::slice::from_raw_parts(data, len + 1).to_vec()
        };
        Self(core::cell::UnsafeCell::new((buf, len)))
      }
      pub fn display<'a>(&'a self) -> $display<'a> {
        self.as_ref().display()
      }
    }
    impl From<&[$type]> for $name {
      fn from(value: &[$type]) -> Self {
        let mut buf = value.to_vec();
        let len = value.iter().take_while(|c| **c != 0).count();
        if len == value.len() {
          buf.push(0);
        }
        Self(core::cell::UnsafeCell::new((buf, len)))
      }
    }
    impl<const N: usize> From<&[$type; N]> for $name {
      fn from(value: &[$type; N]) -> Self {
        let mut buf = value.to_vec();
        let len = value.iter().take_while(|c| **c != 0).count();
        if len == value.len() {
          buf.push(0);
        }
        Self(core::cell::UnsafeCell::new((buf, len)))
      }
    }
    impl From<$name> for Vec<$type> {
      fn from(value: $name) -> Self {
        value.0.into_inner().0
      }
    }
    impl From<Vec<$type>> for $name {
      fn from(mut buf: Vec<$type>) -> Self {
        let len = buf.iter().take_while(|c| **c != 0).count();
        if len == buf.len() {
          buf.push(0);
        }
        Self(core::cell::UnsafeCell::new((buf, len)))
      }
    }
    impl AsRef<$asref> for $name {
      fn as_ref(&self) -> &$asref {
        self.refresh();
        unsafe { <$asref>::from_slice_unchecked(&self.inner().0) }
      }
    }
    impl AsMut<$asref> for $name {
      fn as_mut(&mut self) -> &mut $asref {
        self.refresh();
        unsafe { <$asref>::from_mut_slice_unchecked(&mut self.inner().0) }
      }
    }
    impl core::borrow::Borrow<$asref> for $name {
      fn borrow(&self) -> &$asref {
        self.as_ref()
      }
    }
    impl Default for $name {
      fn default() -> Self {
        Self::new()
      }
    }
    impl Clone for $name {
      fn clone(&self) -> Self {
        self.refresh();
        let (buf, len) = self.inner();
        Self(core::cell::UnsafeCell::new((buf.clone(), *len)))
      }
    }
  };
}

macro_rules! common_str_writes_impl {
  ($name:ty, $fn:ident) => {
    impl core::fmt::Write for &mut $name {
      fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let buf = s.as_bytes();
        let writable = self.0.len() - 1;
        let valid_bytes = if let Err(err) = $crate::strings::internals::check_is_valid_utf8(buf) {
          err.valid_up_to()
        } else {
          buf.len()
        };
        let mut buf = &buf[0..valid_bytes];
        use $crate::ignore::ResultIgnoreExt;
        let chars_len = unsafe { $crate::strings::internals::$fn(buf) }.ignore();
        if chars_len > writable {
          return Err(core::fmt::Error);
        }
        let written = chars_len;
        type CharType = <$name as $crate::strings::CStrCharType>::Char;
        for i in 0..written {
          let (cp, rest) = unsafe { $crate::strings::internals::next_code_point(buf).unwrap() };
          self.0[i] = cp as CharType;
          buf = rest;
        }
        self.0[written + 1] = 0;
        Ok(())
      }
    }
    #[cfg(not(feature = "no_std"))]
    impl std::io::Write for &mut $name {
      fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let writable = self.0.len() - 1;
        let valid_bytes = if let Err(err) = $crate::strings::internals::check_is_valid_utf8(buf) {
          err.valid_up_to()
        } else {
          buf.len()
        };
        let mut buf = &buf[0..valid_bytes];
        use $crate::ignore::ResultIgnoreExt;
        let chars_len = unsafe { $crate::strings::internals::$fn(buf) }.ignore();
        let written = core::cmp::min(writable, chars_len);
        type CharType = <$name as $crate::strings::CStrCharType>::Char;
        for i in 0..written {
          let (cp, rest) = unsafe { $crate::strings::internals::next_code_point(buf).unwrap() };
          self.0[i] = cp as CharType;
          buf = rest;
        }
        self.0[written + 1] = 0;
        Ok(written)
      }

      fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
      }
    }
  };
}

macro_rules! common_string_writes_impl {
  ($name:ty, $fn:ident) => {
    impl core::fmt::Write for &mut $name {
      fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let buf = s.as_bytes();
        let prev_len = self.refresh();
        let valid_bytes = if let Err(err) = $crate::strings::internals::check_is_valid_utf8(buf) {
          err.valid_up_to()
        } else {
          buf.len()
        };
        let mut buf = &buf[0..valid_bytes];
        if buf.is_empty() {
          return Ok(());
        }
        use $crate::ignore::ResultIgnoreExt;
        let chars_len = unsafe { $crate::strings::internals::$fn(buf) }.ignore();
        let inner = self.inner();
        inner.0.resize(chars_len + prev_len, 0);
        let buffer = &mut inner.0[prev_len..prev_len + chars_len];
        type CharType = <$name as $crate::strings::CStrCharType>::Char;
        for i in 0..chars_len {
          let (cp, rest) = unsafe { $crate::strings::internals::next_code_point(buf).unwrap() };
          buffer[i] = cp as CharType;
          buf = rest;
        }
        inner.0.push(0);
        self.refresh();
        Ok(())
      }
    }
    #[cfg(not(feature = "no_std"))]
    impl std::io::Write for $name {
      fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let prev_len = self.refresh();
        let valid_bytes = if let Err(err) = $crate::strings::internals::check_is_valid_utf8(buf) {
          err.valid_up_to()
        } else {
          buf.len()
        };
        let mut buf = &buf[0..valid_bytes];
        if buf.is_empty() {
          return Ok(0);
        }
        use $crate::ignore::ResultIgnoreExt;
        let chars_len = unsafe { $crate::strings::internals::$fn(buf) }.ignore();
        let inner = self.inner();
        inner.0.resize(chars_len + prev_len, 0);
        let buffer = &mut inner.0[prev_len..prev_len + chars_len];
        type CharType = <$name as $crate::strings::CStrCharType>::Char;
        for i in 0..chars_len {
          let (cp, rest) = unsafe { $crate::strings::internals::next_code_point(buf).unwrap() };
          buffer[i] = cp as CharType;
          buf = rest;
        }
        inner.0.push(0);
        self.refresh();
        Ok(valid_bytes)
      }

      fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
      }
    }
  };
}

pub(super) use common_cstr_impls;
pub(super) use common_cstring_impls;
pub(super) use common_str_writes_impl;
pub(super) use common_string_writes_impl;
