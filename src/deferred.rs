pub struct Deferred<T: FnOnce()>(core::option::Option<T>);

impl<T: FnOnce()> Deferred<T> {
  #[must_use]
  pub fn new(closure: T) -> Self {
    Self(Some(closure))
  }
  pub fn run(self) {
    drop(self)
  }
  pub fn forget(mut self) {
    self.0.take();
  }
}

impl<T: FnOnce()> core::ops::Drop for Deferred<T> {
  fn drop(&mut self) {
    let Some(closure) = self.0.take() else {return};
    closure()
  }
}

#[macro_export]
macro_rules! defer {
  (<- move $($code:tt)*) => {
    $crate::deferred::Deferred::new(move ||{$($code)*})
  };
  ($handle:ident <- move $($code:tt)*) => {
    let $handle = $crate::deferred::Deferred::new(move ||{$($code)*});
  };
  (move $($code:tt)*) => {
    let _tmp = $crate::deferred::Deferred::new(move ||{$($code)*});
  };
  (<- $($code:tt)*) => {
    $crate::deferred::Deferred::new(||{$($code)*})
  };
  ($handle:ident <- $($code:tt)*) => {
    let $handle = $crate::deferred::Deferred::new(||{$($code)*});
  };
  ($($code:tt)*) => {
    let _tmp = $crate::deferred::Deferred::new(||{$($code)*});
  };
}

#[macro_export]
macro_rules! unsafe_defer {
  (<- move $($code:tt)*) => {
    $crate::deferred::Deferred::new(move ||unsafe {$($code)*})
  };
  ($handle:ident <- move $($code:tt)*) => {
    let $handle = $crate::deferred::Deferred::new(move ||unsafe {$($code)*});
  };
  (move $($code:tt)*) => {
    let _tmp = $crate::deferred::Deferred::new(move ||unsafe {$($code)*});
  };
  (<- $($code:tt)*) => {
    $crate::deferred::Deferred::new(||unsafe {$($code)*})
  };
  ($handle:ident <- $($code:tt)*) => {
    let $handle = $crate::deferred::Deferred::new(||unsafe {$($code)*});
  };
  ($($code:tt)*) => {
    let _tmp = $crate::deferred::Deferred::new(||unsafe {$($code)*});
  };
}

#[cfg(test)]
mod tests {
  #[test]
  fn unsafe_defers() {
    let a = core::cell::UnsafeCell::new(1);
    let scope = || {
      assert_eq!(unsafe { *a.get() }, 1);
      unsafe_defer! {
        *a.get() = 3;
      };
      assert_eq!(unsafe { *a.get() }, 1);
    };
    scope();
    assert_eq!(unsafe { *a.get() }, 3);
  }
  #[test]
  fn defers() {
    let a = std::cell::Cell::new(1);
    let scope = || {
      assert_eq!(a.get(), 1);
      defer! {
        a.set(3);
      };
      assert_eq!(a.get(), 1);
    };
    scope();
    assert_eq!(a.get(), 3);
  }
  #[test]
  fn defers_stored() {
    let a = std::cell::Cell::new(1);
    let scope = || {
      assert_eq!(a.get(), 1);
      defer! { deferred <-
        a.set(3);
      };
      assert_eq!(a.get(), 1);
      deferred.run();
    };
    scope();
    assert_eq!(a.get(), 3);
  }

  #[test]
  fn defers_stored_outside() {
    let a = std::cell::Cell::new(1);
    let scope = || {
      assert_eq!(a.get(), 1);
      let deferred = defer! { <-
        a.set(3);
      };
      assert_eq!(a.get(), 1);
      deferred.run();
    };
    scope();
    assert_eq!(a.get(), 3);
  }
  #[test]
  fn forgets_stored_outside() {
    let a = std::cell::Cell::new(1);
    let scope = || {
      assert_eq!(a.get(), 1);
      let deferred = defer! { <-
        a.set(3);
      };
      assert_eq!(a.get(), 1);
      deferred.forget();
    };
    scope();
    assert_eq!(a.get(), 1);
  }

  #[test]
  fn forgets() {
    let a = std::cell::Cell::new(1);
    let scope = || {
      assert_eq!(a.get(), 1);
      defer! { deferred <-
        a.set(3);
      };
      assert_eq!(a.get(), 1);
      deferred.forget();
    };
    scope();
    assert_eq!(a.get(), 1);
  }
  #[test]
  fn unsafe_defers_move() {
    let mut a: i32 = 1;
    let a_ptr = &mut a as *mut _;
    let mut scope = || {
      assert_eq!(a, 1);
      unsafe_defer! { move
        *a_ptr = 3;
      };
      a = 2;
      assert_eq!(a, 2);
    };
    scope();
    assert_eq!(a, 3);
  }
}
