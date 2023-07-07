macro_rules! common_staticcstr_impls {
  ($name:ident, $type:ty, $into:ty, $asref:ty, $display:ident, $iter:ident) => {
    /// A static str contains it's data on the stack
    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct $name<const CAPACITY: usize>([$type; CAPACITY], [$type; 1]);
    impl<const CAPACITY: usize> $crate::strings::CStrCharType for $name<CAPACITY> {
      type Char = $type;
    }
    impl<const CAPACITY: usize> $name<CAPACITY> {
      /// Total capacity of static str
      /// NOTE: extra nul character is not included
      pub const CAPACITY: usize = CAPACITY;
      /// Total capacity of static str as DWORD
      /// NOTE: extra nul character is not included
      pub const CAPACITY_DWORD: u32 = CAPACITY as u32;

      /// Constructs new empty instance
      pub fn zeroed() -> Self {
        Self([0 as $type; CAPACITY], [0])
      }

      // Waiting for https://github.com/rust-lang/rust/issues/8995 to be stabilized
      // pub type Char = $type;
      // For now working around using trait

      /// Calculates the length of static str by iterating over it's contents
      /// searching for nul-terminator character
      /// NOTE: it will never exceed the total `CAPACITY` but can be equal to it
      pub const fn len_usize(&self) -> usize {
        let mut items = self.0.as_slice();
        let mut i = 0;
        while let Some(item) = items.first() {
          if *item == 0 {
            return i;
          }
          items = match items {
            [_, rest @ ..] => rest,
            [] => break,
          };
          i += 1
        }
        i
      }
      /// Checks wheither the static str is empty
      /// i.e. starts with nul-terminator character
      pub const fn is_empty(&self) -> bool {
        self.len_usize() == 0
      }
      /// Calculates the length of static str by iterating over it's contents
      /// searching for nul-terminator character.
      /// NOTE: This method returns the result as DWORD
      /// NOTE: it will never exceed the total `CAPACITY`
      pub const fn len_dword(&self) -> u32 {
        self.len_usize() as u32
      }
      /// Calculates the length of static str by iterating over it's contents
      /// searching for nul-terminator character.
      /// NOTE: This method casts calculated length from usize to the desired type via TryInto trait
      /// NOTE: it will never exceed the total `CAPACITY`
      pub fn len<T: TryFrom<usize> + Default>(&self) -> T {
        self.len_usize().try_into().unwrap_or_default()
      }
      /// Calculates the size in bytes of contents including nul-terminator character
      pub fn sizeof_usize(&self) -> usize {
        (self.len_with_nul_usize() * core::mem::size_of::<$type>())
      }
      /// Calculates the size in bytes of contents including nul-terminator character.
      /// NOTE: This method casts the result to DWORD
      pub fn sizeof_dword(&self) -> u32 {
        (self.len_with_nul_dword() * core::mem::size_of::<$type>() as u32)
      }
      /// Calculates the size in bytes of contents including nul-terminator character.
      /// NOTE: This method casts the result to the desired type via TryInto trait
      pub fn sizeof<T: TryFrom<usize> + Default>(&self) -> T {
        (self.len_with_nul_usize() * core::mem::size_of::<$type>())
          .try_into()
          .unwrap_or_default()
      }
      /// Calculates the length of the static str including nul-terminator character.
      pub const fn len_with_nul_usize(&self) -> usize {
        self.len_usize() + 1
      }
      /// Calculates the length of the static str including nul-terminator character.
      /// NOTE: This method casts the result to DWORD
      pub const fn len_with_nul_dword(&self) -> u32 {
        self.len_dword() + 1
      }
      /// Calculates the length of the static str including nul-terminator character.
      /// NOTE: This method casts the result to the desired type via TryInto trait
      pub fn len_with_nul<T: TryFrom<usize> + Default>(&self) -> T {
        (self.len_usize() + 1).try_into().unwrap_or_default()
      }
      /// Returns the total capacity (not including extra nul-terminator character)
      /// NOTE: This method casts the result to the desired type via TryInto trait
      pub fn capacity<T: TryFrom<usize> + Default>(&self) -> T {
        Self::CAPACITY.try_into().unwrap_or_default()
      }
      /// Returns the contents of the static str until nul-terminator (not including) as immutable slice
      pub const fn as_slice(&self) -> &[$type] {
        // Const implementation of: "&self.0[0..self.len_usize()]""
        unsafe { core::slice::from_raw_parts(self.0.as_ptr(), self.len_usize()) }
      }
      /// Returns the contents of the static str until nul-terminator (not including) as mutable slice
      pub fn as_mut_slice(&mut self) -> &mut [$type] {
        let len = self.len_usize();
        &mut self.0[0..len]
      }
      /// Returns the contents of the static str until nul-terminator (and including it) as immutable slice
      pub const fn as_slice_with_nul(&self) -> &[$type] {
        // Const implementation of: "&self.0[0..self.len_with_nul_usize()]"
        unsafe { core::slice::from_raw_parts(self.0.as_ptr(), self.len_with_nul_usize()) }
      }
      /// Returns the contents of the static str until nul-terminator (and including it) as mutable slice
      /// SAFETY: caller should not mutate the extra nul-termianator at position `CAPACITY + 1`
      pub unsafe fn as_mut_slice_with_nul(&mut self) -> &mut [$type] {
        let len = self.len_with_nul_usize();
        core::slice::from_raw_parts_mut(self.0.as_mut_ptr(), len)
      }
      /// Returns all contents of the static str as immutable slice
      /// NOTE: extra nul-terminator at the end is not included
      pub const fn as_slice_full(&self) -> &[$type; CAPACITY] {
        &self.0
      }
      /// Returns all contents of the static str as mutable slice
      /// NOTE: extra nul-terminator at the end is not included therefore this method is safe
      pub fn as_mut_slice_full(&mut self) -> &mut [$type; CAPACITY] {
        &mut self.0
      }
      /// Returns the const pointer to the contents
      /// NOTE: string represented by the returned pointer is always nul-terminated
      /// and is at most `CAPACITY` characters long (not including nul-terminator)
      pub const fn as_ptr(&self) -> *const $type {
        self.0.as_ptr()
      }
      /// Returns the mutable pointer to the contents
      /// SAFETY: extra nul-terminator at position `return value`.offset(1) should not be altered
      pub fn as_mut_ptr(&mut self) -> *mut $type {
        self.0.as_mut_ptr()
      }
      /// Returns the mutable pointer to the contents given shared reference
      /// SAFETY: although the returned pointer is mutable, the string represented by it should not be altered
      /// NOTE: this method is useful for FFI functions that does not require string mutation, however does take mutable pointer
      pub const unsafe fn as_mut_ptr_bypass(&self) -> *mut $type {
        self.0.as_ptr() as *mut _
      }
      /// Returns the character at a given index
      /// NOTE: this method returns None in case of `index` beeing greater then or equal to `CAPACITY`
      pub const fn get(&self, index: usize) -> Option<$type> {
        if index >= CAPACITY {
          return None;
        }
        Some(unsafe { *self.0.as_ptr().add(index) })
      }
      /// Returns the character at a given index
      /// NOTE: this method returns None in case of `index` beeing greater then or equal to `CAPACITY`
      pub const fn get_ref(&self, index: usize) -> Option<&$type> {
        if index >= CAPACITY {
          return None;
        }
        Some(unsafe { &*self.0.as_ptr().add(index) })
      }
      /// Returns mutable reference to the character at a given index
      /// NOTE: this method returns None in case of `index` beeing greater then or equal to `CAPACITY`
      pub fn get_mut(&mut self, index: usize) -> Option<&mut $type> {
        self.0.get_mut(index)
      }
      /// Returns the substring of the static str
      pub const fn range(&self, range: core::ops::RangeFrom<usize>) -> &$asref {
        if range.start > CAPACITY {
          unsafe { <$asref>::from_slice_unchecked(&self.1) }
        } else {
          unsafe {
            <$asref>::from_ptr_unchecked(
              self.0.as_ptr().add(range.start),
              CAPACITY + 1 - range.start,
            )
          }
        }
      }
      /// Returns the mutable substring of the static str
      pub fn range_mut(&mut self, range: core::ops::RangeFrom<usize>) -> &mut $asref {
        if range.start >= CAPACITY {
          unsafe { <$asref>::from_mut_slice_unchecked(&mut self.1) }
        } else {
          let len = CAPACITY - range.start;
          unsafe { <$asref>::from_mut_ptr_unchecked(self.0[range].as_mut_ptr(), len) }
        }
      }
      /// Copies data from slice to the static string and returns it
      /// NOTE: panics if slice is longer then `CAPACITY`
      pub const fn from_slice(mut data: &[$type]) -> Self {
        let mut array = [0 as $type; CAPACITY];
        let mut i = 0;
        loop {
          (array[i], data) = match data {
            [first, rest @ ..] => (*first, rest),
            [] => break,
          };
          i += 1;
        }
        // Waiting for `core::slice::from_raw_parts_mut` to be avaliable at compile time
        // ```
        // let slice = core::slice::from_raw_parts_mut(array.as_mut_ptr(), data.len());
        // slice.copy_from_slice(data);
        // ```
        Self(array, [0])
      }
      /// Copies data from data pointed to by `data` to the static string and returns it
      /// NOTE: If pointed to data does not contain nul-terminator in first `CAPACITY+1` characters,
      /// then Err is returned
      pub const unsafe fn from_ptr(data: *const $type) -> Result<Self, $crate::strings::StrError> {
        let mut inf_buf = core::slice::from_raw_parts(data, CAPACITY + 1);
        let mut array = [0 as $type; CAPACITY];
        // Const implementation of: "let len = inf_buf.iter().take_while(|c| **c != 0).count();"
        // -----
        let mut i = 0;
        while let Some(item) = inf_buf.first() {
          if *item == 0 {
            break;
          }
          (array[i], inf_buf) = match inf_buf {
            [first, rest @ ..] => (*first, rest),
            [] => return Err($crate::strings::StrError::NulNotFound),
          };
          i += 1;
        }
        // -----
        // Waiting for `core::slice::from_raw_parts_mut` to be avaliable at compile time
        // ```
        // let buf = core::slice::from_raw_parts(data, i);
        // let slice = core::slice::from_raw_parts_mut(array.as_mut_ptr(), i);
        // slice.copy_from_slice(buf);
        // ```
        Ok(Self(array, [0]))
      }
      /// Copies data from data pointed to by `data` to the static string and returns it
      /// NOTE: this function copies data until it reaches nul-terminator or until it copies `len` characters
      pub const unsafe fn from_ptr_unchecked(data: *const $type, len: usize) -> Self {
        let mut buf = core::slice::from_raw_parts(data, len);
        let mut array = [0 as $type; CAPACITY];
        let mut i = 0;
        while let Some(item) = buf.first() {
          if *item == 0 {
            break;
          }
          (array[i], buf) = match buf {
            [first, rest @ ..] => (*first, rest),
            [] => break,
          };
          i += 1;
        }
        // Waiting for `core::slice::from_raw_parts_mut` to be avaliable at compile time
        // ```
        // let slice = core::slice::from_raw_parts_mut(array.as_mut_ptr(), len);
        // slice.copy_from_slice(buf);
        // ```
        Self(array, [0])
      }
      /// Copies data from data pointed to by `data` to the static string and returns it
      /// NOTE: this function returns Err in case of nul-terminator was not found in first `max_len` characters
      pub const unsafe fn from_ptr_n(
        data: *const $type,
        max_len: usize,
      ) -> Result<Self, $crate::strings::StrError> {
        let mut buf = core::slice::from_raw_parts(data, max_len);
        let mut array = [0 as $type; CAPACITY];
        let mut i = 0;
        while let Some(item) = buf.first() {
          if *item == 0 {
            break;
          }
          (array[i], buf) = match buf {
            [first, rest @ ..] => (*first, rest),
            [] => return Err($crate::strings::StrError::NulNotFound),
          };
          i += 1;
        }
        // Waiting for `core::slice::from_raw_parts_mut` to be avaliable at compile time
        // ```
        // let slice = core::slice::from_raw_parts_mut(array.as_mut_ptr(), len);
        // slice.copy_from_slice(buf);
        // ```
        Ok(Self(array, [0]))
      }
      /// Returns display wrapper for static str
      pub fn display<'a>(&'a self) -> $display<'a> {
        $display(&self.0[0..self.len_usize()])
      }
      /// Tries to construct static str from slice
      /// NOTE: in case of slice does not contain nul-terminator in
      /// first `CAPACITY` characters this function returns Err
      pub const fn try_from_slice(value: &[$type]) -> Result<Self, $crate::strings::StrError> {
        // let len = value.iter().take_while(|c| **c != 0).count();
        let mut buf = value;
        let mut len = 0;
        while let Some(item) = buf.first() {
          if *item == 0 {
            break;
          }
          buf = match buf {
            [_, rest @ ..] => rest,
            [] => return Err($crate::strings::StrError::NulNotFound),
          };
          len += 1;
        }
        if len == value.len() || len >= CAPACITY {
          Err($crate::strings::StrError::NulNotFound)
        } else {
          Ok(Self::from_slice(value))
        }
      }
      /// Returns an iterator over characters of the static str
      /// until nul-terminator
      /// NOTE: Items are returned by shared reference
      pub fn iter(&self) -> $iter<&$name<CAPACITY>> {
        $iter(self, 0)
      }
      /// Returns an iterator over characters of the static str
      /// until nul-terminator
      /// NOTE: Items are returned by mutable reference
      pub fn iter_mut(&mut self) -> $iter<&mut $name<CAPACITY>> {
        $iter(self, 0)
      }
      /// Returns an iterator over characters of the static str
      /// until nul-terminator
      /// NOTE: Items are returned by value
      pub fn into_iter(self) -> $iter<$name<CAPACITY>> {
        $iter(self, 0)
      }
    }
    impl<const CAPACITY: usize> Default for $name<CAPACITY> {
      fn default() -> Self {
        Self::zeroed()
      }
    }
    impl<const CAPACITY: usize> TryFrom<&[$type]> for $name<CAPACITY> {
      type Error = $crate::strings::StrError;
      fn try_from(value: &[$type]) -> Result<Self, Self::Error> {
        Self::try_from_slice(value)
      }
    }
    impl<const CAPACITY: usize, const N: usize> TryFrom<&[$type; N]> for $name<CAPACITY> {
      type Error = $crate::strings::StrError;
      fn try_from(value: &[$type; N]) -> Result<Self, $crate::strings::StrError> {
        Self::try_from_slice(value)
      }
    }
    impl<const CAPACITY: usize> From<$name<CAPACITY>> for $into {
      fn from(value: $name<CAPACITY>) -> Self {
        Self::from(&value.0)
      }
    }
    impl<const CAPACITY: usize> AsRef<$name<CAPACITY>> for &$name<CAPACITY> {
      fn as_ref(&self) -> &$name<CAPACITY> {
        self
      }
    }
    impl<const CAPACITY: usize> AsMut<$name<CAPACITY>> for &mut $name<CAPACITY> {
      fn as_mut(&mut self) -> &mut $name<CAPACITY> {
        self
      }
    }
    impl<const CAPACITY: usize> AsRef<$asref> for $name<CAPACITY> {
      fn as_ref(&self) -> &$asref {
        unsafe { <$asref>::from_ptr_unchecked(self.0.as_ptr(), CAPACITY + 1) }
      }
    }
    impl<const CAPACITY: usize> AsMut<$asref> for $name<CAPACITY> {
      fn as_mut(&mut self) -> &mut $asref {
        unsafe { <$asref>::from_mut_ptr_unchecked(self.0.as_mut_ptr(), CAPACITY + 1) }
      }
    }
    impl<const CAPACITY: usize> core::borrow::Borrow<$asref> for $name<CAPACITY> {
      fn borrow(&self) -> &$asref {
        self.as_ref()
      }
    }
    impl<const CAPACITY: usize> core::borrow::BorrowMut<$asref> for $name<CAPACITY> {
      fn borrow_mut(&mut self) -> &mut $asref {
        self.as_mut()
      }
    }
    impl<const CAPACITY: usize> core::ops::Deref for $name<CAPACITY> {
      type Target = $asref;

      #[inline]
      fn deref(&self) -> &$asref {
        self.as_ref()
      }
    }
    impl<const CAPACITY: usize> core::ops::DerefMut for $name<CAPACITY> {
      #[inline]
      fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
      }
    }
    impl<const CAPACITY: usize> core::ops::Index<usize> for $name<CAPACITY> {
      type Output = $type;
      #[inline]
      fn index(&self, index: usize) -> &Self::Output {
        self.get_ref(index).unwrap()
      }
    }
    impl<const CAPACITY: usize> core::ops::IndexMut<usize> for $name<CAPACITY> {
      #[inline]
      fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
      }
    }
    impl<const CAPACITY: usize> core::ops::Index<core::ops::RangeFrom<usize>> for $name<CAPACITY> {
      type Output = $asref;
      #[inline]
      fn index(&self, index: core::ops::RangeFrom<usize>) -> &Self::Output {
        self.range(index)
      }
    }
    impl<const CAPACITY: usize> core::ops::IndexMut<core::ops::RangeFrom<usize>>
      for $name<CAPACITY>
    {
      #[inline]
      fn index_mut(&mut self, index: core::ops::RangeFrom<usize>) -> &mut Self::Output {
        self.range_mut(index)
      }
    }
    /// A call to `$name::into_iter` or `$name::iter` or `$name::iter_mut` returns an instance of this class
    /// It can be used to iterate over characters of static str until nul-terminator
    pub struct $iter<T>(T, usize);
    impl<const CAPACITY: usize> core::iter::Iterator for $iter<$name<CAPACITY>> {
      type Item = $type;
      fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let ret = self.0.get(self.1)?;
        if ret == 0 {
          return None;
        }
        self.1 += 1;
        Some(ret)
      }
    }
    impl<'col, const CAPACITY: usize> core::iter::Iterator for $iter<&'col $name<CAPACITY>> {
      type Item = &'col $type;
      fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let ret = self.0.get_ref(self.1)?;
        if *ret == 0 {
          return None;
        }
        self.1 += 1;
        Some(ret)
      }
    }
    impl<'col, const CAPACITY: usize> core::iter::Iterator for $iter<&'col mut $name<CAPACITY>> {
      type Item = &'col mut $type;
      fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let ret = self.0.get_mut(self.1)?;
        if *ret == 0 {
          return None;
        }
        self.1 += 1;
        Some(unsafe { &mut *(ret as *mut _) })
      }
    }
    impl<const CAPACITY: usize> core::iter::IntoIterator for $name<CAPACITY> {
      type Item = $type;
      type IntoIter = $iter<$name<CAPACITY>>;
      fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
      }
    }
    impl<const CAP1: usize, const CAP2: usize> core::cmp::PartialEq<$name<CAP1>> for $name<CAP2> {
      fn eq(&self, rhs: &$name<CAP1>) -> bool {
        let first = self
          .iter()
          .take_while(|a| **a != 0)
          .map(Some)
          .chain(core::iter::once(None));
        let second = rhs
          .iter()
          .take_while(|a| **a != 0)
          .map(Some)
          .chain(core::iter::once(None));
        match first.zip(second).take_while(|(a, b)| (a == b)).last() {
          Some((a, b)) if a == b => true,
          _ => false,
        }
      }
    }
    impl<const CAP: usize> core::cmp::Eq for $name<CAP> {}
  };
}

macro_rules! common_cstr_impls {
  ($name:ident, $type:ty, $into:ty, $display:ident, $iter:ident, $static:ident) => {
    /// A wrapper struct for slice of characters
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct $name([$type]);
    impl $crate::strings::CStrCharType for $name {
      type Char = $type;
    }
    /// A wrapper for cstring display
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

      /// Checks wheither a cstr slice is empty
      pub const fn is_empty(&self) -> bool {
        self.len_usize() == 0
      }
      /// Calculates the length of a cstr slice by
      /// iterating over characters in search of nul-terminator
      pub const fn len_usize(&self) -> usize {
        let mut items = &self.0;
        let mut i = 0;
        while let Some(item) = items.first() {
          if *item == 0 {
            return i;
          }
          items = match items {
            [_, rest @ ..] => rest,
            [] => break,
          };
          i += 1
        }
        panic!("CStr was in invalid state. Missing nul-terminator")
      }
      /// Calculates the length of a cstr slice by
      /// iterating over characters in search of nul-terminator
      /// NOTE: this method returns value as DWORD
      pub const fn len_dword(&self) -> u32 {
        self.len_usize() as u32
      }
      /// Calculates the length of a cstr slice by
      /// iterating over characters in search of nul-terminator
      /// NOTE: This method casts the result to the desired type via TryInto trait
      pub fn len<T: TryFrom<usize> + Default>(&self) -> T {
        self.len_usize().try_into().unwrap_or_default()
      }
      /// Calculates the length of a cstr slice including nul-terminator by
      /// iterating over characters in search of nul-terminator
      pub const fn len_with_nul_usize(&self) -> usize {
        self.len_usize() + 1
      }
      /// Calculates the length of a cstr slice including nul-terminator by
      /// iterating over characters in search of nul-terminator
      /// NOTE: this method returns value as DWORD
      pub const fn len_with_nul_dword(&self) -> u32 {
        self.len_dword() + 1
      }
      /// Calculates the length of a cstr slice including nul-terminator by
      /// iterating over characters in search of nul-terminator
      /// NOTE: This method casts the result to the desired type via TryInto trait
      pub fn len_with_nul<T: TryFrom<usize> + Default>(&self) -> T {
        (self.len_usize() + 1).try_into().unwrap_or_default()
      }
      /// Calculates the size in bytes of contents including nul-terminator character
      pub fn sizeof_usize(&self) -> usize {
        (self.len_with_nul_usize() * core::mem::size_of::<$type>())
      }
      /// Calculates the size in bytes of contents including nul-terminator character
      /// NOTE: this method returns value as DWORD
      pub fn sizeof_dword(&self) -> u32 {
        (self.len_with_nul_dword() * core::mem::size_of::<$type>() as u32)
      }
      /// Calculates the size in bytes of contents including nul-terminator character
      /// NOTE: This method casts the result to the desired type via TryInto trait
      pub fn sizeof<T: TryFrom<usize> + Default>(&self) -> T {
        (self.len_with_nul_usize() * core::mem::size_of::<$type>())
          .try_into()
          .unwrap_or_default()
      }
      /// Returns the total length of underlying slice (excluding nul-terminator)
      pub const fn capacity_usize(&self) -> usize {
        self.0.len() - 1
      }
      /// Returns the total length of underlying slice (excluding nul-terminator)
      /// NOTE: this method returns value as DWORD
      pub const fn capacity_dword(&self) -> u32 {
        self.capacity_usize() as u32
      }
      /// Returns the total length of underlying slice (excluding nul-terminator)
      /// NOTE: This method casts the result to the desired type via TryInto trait
      pub fn capacity<T: TryFrom<usize> + Default>(&self) -> T {
        self.capacity_usize().try_into().unwrap_or_default()
      }
      /// Returns slice representation until nul-terminator (and excluding it)
      pub const fn as_slice(&self) -> &[$type] {
        // Const implementation of: "&self.0[0..self.len_usize()]""
        unsafe { core::slice::from_raw_parts(self.0.as_ptr(), self.len_usize()) }
      }
      /// Returns mutable slice representation until nul-terminator (and excluding it)
      pub fn as_mut_slice(&mut self) -> &mut [$type] {
        let len = self.len_usize();
        unsafe { self.0.get_unchecked_mut(0..len) }
      }
      /// Returns immutable slice representation until nul-terminator (and including it)
      pub const fn as_slice_with_nul(&self) -> &[$type] {
        // Const implementation of: "&self.0[0..self.len_with_nul_usize()]"
        unsafe { core::slice::from_raw_parts(self.0.as_ptr(), self.len_with_nul_usize()) }
      }
      /// Returns immutable slice representation until nul-terminator (and including it)
      /// SAFETY: caller should not mutate the last character in slice, i.e. nul-terminator
      pub unsafe fn as_mut_slice_with_nul(&mut self) -> &mut [$type] {
        let len = self.len_with_nul_usize();
        self.0.get_unchecked_mut(0..len)
      }
      /// Returns a reference to the full underlying slice
      pub const fn as_slice_full(&self) -> &[$type] {
        &self.0
      }
      /// Returns a mutable reference to the full underlying slice (excluding the last characher to make it safe)
      pub fn as_mut_slice_full(&mut self) -> &mut [$type] {
        unsafe { self.0.get_unchecked_mut(..self.0.len()-1) }
      }
      /// Returns a const pointer to the underlying data
      pub const fn as_ptr(&self) -> *const $type {
        self.0.as_ptr()
      }
      /// Returns a mutable pointer to the underlying data
      pub fn as_mut_ptr(&mut self) -> *mut $type {
        self.0.as_mut_ptr()
      }
      /// Returns the mutable pointer to the contents given shared reference
      /// SAFETY: although the returned pointer is mutable, the string represented by it should not be altered
      /// NOTE: this method is useful for FFI functions that does not require string mutation, however does take mutable pointer
      pub const unsafe fn as_mut_ptr_bypass(&self) -> *mut $type {
        self.0.as_ptr() as *mut _
      }
      /// Returns the character at a given index
      /// NOTE: this method returns None in case of `index` is out of bounds of the underlying slice
      pub const fn get(&self, index: usize) -> Option<$type> {
        if index >= self.0.len() {
          return None;
        }
        Some(unsafe { *self.0.as_ptr().add(index) })
      }
      /// Returns the character at a given index
      /// NOTE: this method returns None in case of `index` is out of bounds of the underlying slice
      pub const fn get_ref(&self, index: usize) -> Option<&$type> {
        if index >= self.0.len() {
          return None;
        }
        Some(unsafe { &*self.0.as_ptr().add(index) })
      }
      /// Returns mutable reference to the character at a given index
      /// NOTE: this method returns None in case of `index` is out of bounds of the underlying slice
      /// NOTE: this method does not provide access to the last character, i.e. nul-terminator
      pub fn get_mut(&mut self, index: usize) -> Option<&mut $type> {
        if index + 1 >= self.0.len() {
          return None;
        }
        Some(unsafe { self.0.get_unchecked_mut(index) })
      }
      /// Returns the substring of the cstr
      pub const fn range(&self, range: core::ops::RangeFrom<usize>) -> &Self {
        if range.start >= self.0.len() {
          unsafe { Self::from_ptr_unchecked(self.0.as_ptr().add(self.0.len()-1), 1) }
        } else {
          let len = self.0.len() - range.start;
          unsafe {
            Self::from_ptr_unchecked(self.0.as_ptr().add(range.start), len)
          }
        }
      }
      /// Returns the mutable substring of the cstr
      /// NOTE: this method does not provide access to the last character, i.e. nul-terminator
      pub fn range_mut(&mut self, range: core::ops::RangeFrom<usize>) -> &mut Self {
        if range.start + 1 >= self.0.len() {
          unsafe { Self::from_mut_ptr_unchecked(self.0.as_mut_ptr().add(self.0.len()-1), 1) }
        } else {
          let len = self.0.len() - 1 - range.start;
          unsafe { Self::from_mut_ptr_unchecked(self.0[range].as_mut_ptr(), len) }
        }
      }
      /// Constructs an instance of immutable cstr given a shared reference to a slice
      /// SAFETY: provided slice should end with a nul-terminator
      pub const unsafe fn from_slice_unchecked(data: &[$type]) -> &Self {
        core::mem::transmute(data)
      }
      /// Constructs an instance of mutable cstr given a mutable reference to a slice
      /// SAFETY: provided slice should end with a nul-terminator
      pub unsafe fn from_mut_slice_unchecked(data: &mut [$type]) -> &mut Self {
        core::mem::transmute(data)
      }
      #[doc = concat!("
      Constructs an instance of immutable cstr given a pointer to a constant string
      SAFETY: `data` should point to a valid memory where
      NOTE: this function can be dangerous because of not constaining length, 
      consider using safer funtion: `", stringify!($name), "::from_ptr_n`
      cstring is stored and this cstring should end with a nul-terminator
      NOTE: lifetime of the returned value is inferred from context
      ")]
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
      #[doc = concat!("
      Constructs an instance of mutable cstr given a pointer to a mutable string
      SAFETY: `data` should point to a valid memory where
      NOTE: this function can be dangerous because of not constaining length, 
      consider using safer funtion: `", stringify!($name), "::from_mut_ptr_n`
      cstring is stored and this cstring should end with a nul-terminator
      NOTE: lifetime of the returned value is inferred from context
      ")]
      pub unsafe fn from_mut_ptr<'a>(data: *mut $type) -> &'a mut Self {
        let inf_buf = core::slice::from_raw_parts(data, usize::MAX);
        let len = inf_buf.iter().take_while(|c| **c != 0).count();
        let buf = core::slice::from_raw_parts_mut(data, len + 1);
        core::mem::transmute(buf)
      }
      /// Constructs an instance of immutable cstr given a pointer to a constant string and it's length
      /// SAFETY: `data` should point to a valid memory where
      /// cstring is stored
      /// PRECOND: *(`data`.add(`capacity`)) == 0, i.e. `data+capacity` should point to the nul-terminator
      /// NOTE: lifetime of the returned value is inferred from context
      pub const unsafe fn from_ptr_unchecked<'a>(data: *const $type, capacity: usize) -> &'a Self {
        let buf = core::slice::from_raw_parts(data, capacity);
        core::mem::transmute(buf)
      }
      /// Constructs an instance of mutable cstr given a pointer to a mutable string and it's length
      /// SAFETY: `data` should point to a valid memory where
      /// cstring is stored
      /// PRECOND: *(`data`.add(`capacity`)) == 0, i.e. `data+capacity` should point to the nul-terminator
      /// NOTE: lifetime of the returned value is inferred from context
      pub unsafe fn from_mut_ptr_unchecked<'a>(data: *mut $type, capacity: usize) -> &'a mut Self {
        let buf = core::slice::from_raw_parts_mut(data, capacity);
        core::mem::transmute(buf)
      }
      /// Constructs an instance of immutable cstr given a pointer to a constant string and a maximum length.
      /// If a string does not contain nul-terminator in first `max_len` characters,
      /// Err is returned by this function
      /// SAFETY: `data` should point to a memory that is valid up to a least `max_len` characters
      /// NOTE: lifetime of the returned value is inferred from context
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
            [] => break,
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
      /// Constructs an instance of mutable cstr given a pointer to a mutable string and a maximum length.
      /// If a string does not contain nul-terminator in first `max_len` characters,
      /// Err is returned by this function
      /// SAFETY: `data` should point to a memory that is valid up to a least `max_len` characters
      /// NOTE: lifetime of the returned value is inferred from context
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
      /// Provides a wrapper that implements core::fmt::Display
      pub fn display<'a>(&'a self) -> $display<'a> {
        $display(&self.0[0..self.len_usize()])
      }
      /// Constructs a mutable cstr from a mutable slice
      /// NOTE: this function returns Err in case of `value` does not end with a nul-terminator
      pub const fn try_from_slice<'a>(value: &'a [$type]) -> Result<&'a Self, $crate::strings::StrError> {
        let Some(&0) = value.last() else {
          return Err($crate::strings::StrError::NulNotFound);
        };
        Ok(unsafe { core::mem::transmute(value) })
      }
      /// Constructs a immutable cstr from a immutable slice
      /// NOTE: this function returns Err in case of `value` does not end with a nul-terminator
      pub fn try_from_mut_slice(
        value: &mut [$type],
      ) -> Result<&mut Self, $crate::strings::StrError> {
        let Some(0) = value.last().copied() else {
          return Err($crate::strings::StrError::NulNotFound);
        };
        Ok(unsafe { core::mem::transmute(value) })
      }
      pub fn try_into_static<const CAPACITY: usize>(&self) -> Result<$static<CAPACITY>, $crate::strings::StrError> {
        if self.len_usize() > CAPACITY {
          return Err($crate::strings::StrError::NulNotFound);
        }
        Ok(<$static<CAPACITY>>::from_slice(&self.0))
      }
    }
    impl<'a> TryFrom<&'a [$type]> for &'a $name {
      type Error = $crate::strings::StrError;
      fn try_from(value: &'a [$type]) -> Result<Self, Self::Error> {
        <$name>::try_from_slice(value)
      }
    }
    impl<'a> TryFrom<&'a mut [$type]> for &'a mut $name {
      type Error = $crate::strings::StrError;
      fn try_from(value: &'a mut [$type]) -> Result<Self, $crate::strings::StrError> {
        <$name>::try_from_mut_slice(value)
      }
    }
    impl<'a, const N: usize> TryFrom<&'a [$type; N]> for &'a $name {
      type Error = $crate::strings::StrError;
      fn try_from(value: &'a [$type; N]) -> Result<Self, $crate::strings::StrError> {
        <$name>::try_from_slice(value)
      }
    }
    impl<'a, const N: usize> TryFrom<&'a mut [$type; N]> for &'a mut $name {
      type Error = $crate::strings::StrError;
      fn try_from(value: &'a mut [$type; N]) -> Result<Self, $crate::strings::StrError> {
        <$name>::try_from_mut_slice(value)
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
    impl core::ops::Index<usize> for &$name {
      type Output = $type;
      #[inline]
      fn index(&self, index: usize) -> &Self::Output {
        self.get_ref(index).unwrap()
      }
    }
    impl core::ops::Index<usize> for &mut $name {
      type Output = $type;
      #[inline]
      fn index(&self, index: usize) -> &Self::Output {
        self.get_ref(index).unwrap()
      }
    }
    impl core::ops::IndexMut<usize> for &mut $name {
      #[inline]
      fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
      }
    }
    impl core::ops::Index<core::ops::RangeFrom<usize>> for &$name {
      type Output = $name;
      #[inline]
      fn index(&self, index: core::ops::RangeFrom<usize>) -> &Self::Output {
        self.range(index)
      }
    }
    impl core::ops::Index<core::ops::RangeFrom<usize>> for &mut $name {
      type Output = $name;
      #[inline]
      fn index(&self, index: core::ops::RangeFrom<usize>) -> &Self::Output {
        self.range(index)
      }
    }
    impl core::ops::IndexMut<core::ops::RangeFrom<usize>> for &mut $name
    {
      #[inline]
      fn index_mut(&mut self, index: core::ops::RangeFrom<usize>) -> &mut Self::Output {
        self.range_mut(index)
      }
    }
    impl<'col> core::iter::Iterator for &'col $name {
      type Item = &'col $type;
      fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let (first, rest) = match &self.0 {
          [0] => return None,
          [first, rest @ ..] => (first, rest),
          _ => unreachable!(),
        };
        *self = unsafe { core::mem::transmute(rest) };
        Some(first)
      }
    }
    impl<'col> core::iter::Iterator for &'col mut $name {
      type Item = &'col mut $type;
      fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let (first, rest) = match &mut self.0 {
          [0] => return None,
          [first, rest @ ..] => (first, rest),
          _ => unreachable!(),
        };
        *self = unsafe { core::mem::transmute(rest) };
        Some(unsafe { &mut *(first as *mut _) })
      }
    }
    impl core::cmp::PartialEq<$name> for $name {
      fn eq(&self, rhs: &$name) -> bool {
        let first = self.take_while(|a|**a!=0).map(Some).chain(core::iter::once(None));
        let second = rhs.take_while(|a|**a!=0).map(Some).chain(core::iter::once(None));
        match first.zip(second).take_while(|(a, b)| (a == b)).last() {
          Some((a, b)) if a == b => true,
          _ => false,
        }
      }
    }
    impl core::cmp::Eq for $name {}
  };
}

macro_rules! common_cstring_impls {
  ($name:ident, $type:ty, $asref:ty, $display:ident, $iter:ident) => {
    pub struct $name(Vec<$type>);
    unsafe impl Send for $name {}
    unsafe impl Sync for $name {}
    impl $crate::strings::CStrCharType for $name {
      type Char = $type;
    }
    impl $name {
      pub fn new() -> Self {
        let mut buf = vec![0 as $type];
        buf.resize(buf.capacity(), 0);
        Self(buf)
      }
      pub fn with_capacity(cap: usize) -> Self {
        let mut buf = vec![0 as $type; cap + 1];
        buf.resize(buf.capacity(), 0);
        Self(buf)
      }
      pub fn reserve<T: TryInto<usize>>(&mut self, total: T) {
        let cap = self.0.capacity();
        self.0.resize(std::cmp::max(total.try_into().unwrap_or(0), cap), 0);
        let cap = self.0.capacity();
        self.0.resize(cap, 0);
      }
      pub fn len_usize(&self) -> usize {
        self.0.iter().take_while(|c|**c != 0).count()
      }
      pub fn len_dword(&self) -> u32 {
        self.len_usize() as u32
      }
      pub fn len<T: TryFrom<usize> + Default>(&self) -> T {
        self.len_usize().try_into().unwrap_or_default()
      }
      pub fn is_empty(&self) -> bool {
        self.len_usize() == 0
      }
      pub fn len_with_nul_usize(&self) -> usize {
        self.len_usize() + 1
      }
      pub fn len_with_nul_dword(&self) -> u32 {
        self.len_dword() + 1
      }
      pub fn len_with_nul<T: TryFrom<usize> + Default>(&self) -> T {
        (self.len_usize() + 1).try_into().unwrap_or_default()
      }
      pub fn sizeof_usize(&self) -> usize {
        (self.len_with_nul_usize() * core::mem::size_of::<$type>())
      }
      pub fn sizeof_dword(&self) -> u32 {
        (self.len_with_nul_dword() * core::mem::size_of::<$type>() as u32)
      }
      pub fn sizeof<T: TryFrom<usize> + Default>(&self) -> T {
        (self.len_with_nul_usize() * core::mem::size_of::<$type>())
          .try_into()
          .unwrap_or_default()
      }
      pub fn capacity_usize(&self) -> usize {
        self.0.len() - 1
      }
      pub fn capacity_dword(&self) -> u32 {
        self.capacity_usize() as u32
      }
      pub fn capacity<T: TryFrom<usize> + Default>(&self) -> T {
        self.capacity_usize().try_into().unwrap_or_default()
      }
      pub fn as_slice(&self) -> &[$type] {
        let len = self.len_usize();
        &self.0[0..len]
      }
      pub unsafe fn as_mut_slice(&mut self) -> &mut [$type] {
        let len = self.len_usize();
        &mut self.0[0..len]
      }
      pub fn as_slice_with_nul(&self) -> &[$type] {
        let len = self.len_with_nul_usize();
        &self.0[0..len]
      }
      pub unsafe fn as_mut_slice_with_nul(&mut self) -> &mut [$type] {
        let len = self.len_with_nul_usize();
        &mut self.0[0..len]
      }
      pub fn as_slice_full(&self) -> &[$type] {
        &self.0
      }
      pub unsafe fn as_mut_slice_full(&mut self) -> &mut [$type] {
        let len = self.0.len() - 1;
        &mut self.0[0..len - 1]
      }
      pub fn as_ptr(&self) -> *const $type {
        self.0.as_ptr()
      }
      pub fn as_mut_ptr(&mut self) -> *mut $type {
        self.0.as_mut_ptr()
      }
      pub unsafe fn from_ptr(data: *const $type) -> Self {
        let inf_buf = core::slice::from_raw_parts(data, usize::MAX);
        let len = inf_buf.iter().take_while(|c| **c != 0).count();
        let buf = core::slice::from_raw_parts(data, len + 1).to_vec();
        Self(buf.to_vec())
      }
      pub unsafe fn from_ptr_unchecked(data: *const $type, len: usize, capacity: usize) -> Self {
        let buf = core::slice::from_raw_parts(data, capacity).to_vec();
        Self(buf.to_vec())
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
    impl core::borrow::BorrowMut<$asref> for $name {
      fn borrow_mut(&mut self) -> &mut $asref {
        self.as_mut()
      }
    }
    impl core::ops::Deref for $name {
      type Target = $asref;

      #[inline]
      fn deref(&self) -> &$asref {
        self.as_ref()
      }
    }
    impl core::ops::DerefMut for $name {
      #[inline]
      fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
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
    // /// A call to `$name::into_iter` or `$name::iter` or `$name::iter_mut` returns an instance of this class
    // /// It can be used to iterate over characters of static str until nul-terminator
    // pub struct $iter<T>(T, usize);
    // impl core::iter::Iterator for $iter<$name> {
    //   type Item = $type;
    //   fn next(&mut self) -> Option<<Self as Iterator>::Item> {
    //     let ret = self.0.get(self.1)?;
    //     if ret == 0 {
    //       return None;
    //     }
    //     self.1 += 1;
    //     Some(ret)
    //   }
    // }
    // impl<'col> core::iter::Iterator for $iter<&'col $name> {
    //   type Item = &'col $type;
    //   fn next(&mut self) -> Option<<Self as Iterator>::Item> {
    //     let ret = self.0.get_ref(self.1)?;
    //     if *ret == 0 {
    //       return None;
    //     }
    //     self.1 += 1;
    //     Some(ret)
    //   }
    // }
    // impl<'col> core::iter::Iterator for $iter<&'col mut $name> {
    //   type Item = &'col mut $type;
    //   fn next(&mut self) -> Option<<Self as Iterator>::Item> {
    //     let ret = self.0.get_mut(self.1)?;
    //     if *ret == 0 {
    //       return None;
    //     }
    //     self.1 += 1;
    //     Some(unsafe { &mut *(ret as *mut _) })
    //   }
    // }
    // impl core::iter::IntoIterator for $name {
    //   type Item = $type;
    //   type IntoIter = $iter<$name>;
    //   fn into_iter(self) -> Self::IntoIter {
    //     self.into_iter()
    //   }
    // }
    // impl core::cmp::PartialEq<$name> for $name {
    //   fn eq(&self, rhs: &$name) -> bool {
    //     let first = self.iter().map(Some).chain(core::iter::once(None));
    //     let second = rhs.iter().map(Some).chain(core::iter::once(None));
    //     match first.zip(second).take_while(|(a, b)| (a == b)).last() {
    //       Some((a, b)) if a == b => true,
    //       _ => false,
    //     }
    //   }
    // }
    // impl core::cmp::Eq for $name {}
  };
}

macro_rules! common_staticstr_writes_impl {
  ($name:ty, $fn:ident) => {
    impl<const CAPACITY: usize> core::fmt::Write for $name {
      fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let buf = s.as_bytes();
        let prev_len = self.len_usize();
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
        if chars_len > CAPACITY - prev_len - 1 {
          return Err(core::fmt::Error);
        }
        let buffer = &mut self.0[prev_len..prev_len + chars_len];
        for i in 0..chars_len {
          let (cp, rest) = unsafe { $crate::strings::internals::next_code_point(buf).unwrap() };
          if cp > <$name as crate::strings::CStrCharType>::Char::MAX as u32 {
            return Err(core::fmt::Error);
          }
          buffer[i] = cp as <$name as crate::strings::CStrCharType>::Char;
          buf = rest;
        }
        self.0[CAPACITY - 1] = 0;
        Ok(())
      }
    }
    #[cfg(not(feature = "no_std"))]
    impl<const CAPACITY: usize> std::io::Write for $name {
      fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let prev_len = self.len_usize();
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
        let written = core::cmp::min(chars_len, Self::CAPACITY - prev_len - 1);
        let buffer = &mut self.0[prev_len..prev_len + written];
        let mut written_len = 0;
        for i in 0..written {
          let (cp, rest) = unsafe { $crate::strings::internals::next_code_point(buf).unwrap() };
          if cp > <$name as crate::strings::CStrCharType>::Char::MAX as u32 {
            return Err(std::io::Error::new(
              std::io::ErrorKind::InvalidData,
              "input buffer contained character that is unrepresentable in target encoding",
            ));
          }
          written_len += unsafe { char::from_u32_unchecked(cp) }.len_utf8();
          buffer[i] = cp as <$name as crate::strings::CStrCharType>::Char;
          buf = rest;
        }
        self.0[Self::CAPACITY - 1] = 0;
        Ok(written_len)
      }

      fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
      }
    }
  };
}

macro_rules! common_str_writes_impl {
  ($name:ty, $fn:ident) => {
    impl core::fmt::Write for &mut $name {
      fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut buf = s.as_bytes();
        use $crate::ignore::ResultIgnoreExt;
        let chars_len = unsafe { $crate::strings::internals::$fn(buf) }.ignore();
        let writable = self.capacity_usize();
        if chars_len > writable {
          return Err(core::fmt::Error);
        }
        let written = chars_len;
        type CharType = <$name as $crate::strings::CStrCharType>::Char;
        for i in 0..written {
          let (cp, rest) = unsafe { $crate::strings::internals::next_code_point(buf).unwrap() };
          if cp > CharType::MAX as u32 {
            return Err(core::fmt::Error);
          }
          self.0[i] = cp as CharType;
          buf = rest;
        }
        self.0[written] = 0;
        *self = unsafe { core::mem::transmute(&mut self.0[written..]) };
        Ok(())
      }
    }
    #[cfg(not(feature = "no_std"))]
    impl std::io::Write for &mut $name {
      fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let valid_bytes = if let Err(err) = $crate::strings::internals::check_is_valid_utf8(buf) {
          err.valid_up_to()
        } else {
          buf.len()
        };
        let mut buf = &buf[0..valid_bytes];
        use $crate::ignore::ResultIgnoreExt;
        let chars_len = unsafe { $crate::strings::internals::$fn(buf) }.ignore();
        let writable = self.capacity_usize();
        let written = core::cmp::min(writable, chars_len);
        type CharType = <$name as $crate::strings::CStrCharType>::Char;
        for i in 0..written {
          let (cp, rest) = unsafe { $crate::strings::internals::next_code_point(buf).unwrap() };
          if cp > CharType::MAX as u32 {
            return Err(std::io::Error::new(
              std::io::ErrorKind::InvalidData,
              "input buffer contained character that is unrepresentable in target encoding",
            ));
          }
          self.0[i] = cp as CharType;
          buf = rest;
        }
        self.0[written] = 0;
        *self = unsafe { core::mem::transmute(&mut self.0[written..]) };
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
          if cp > CharType::MAX as u32 {
            return Err(core::fmt::Error);
          }
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
          if cp > CharType::MAX as u32 {
            return Err(std::io::Error::new(
              std::io::ErrorKind::InvalidData,
              "input buffer contained character that is unrepresentable in target encoding",
            ));
          }
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
pub(super) use common_staticcstr_impls;
pub(super) use common_staticstr_writes_impl;
pub(super) use common_str_writes_impl;
pub(super) use common_string_writes_impl;
