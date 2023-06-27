pub struct Deferred<T: FnOnce()>(Option<T>);

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

impl<T: FnOnce()> Drop for Deferred<T> {
  fn drop(&mut self) {
    let Some(closure) = self.0.take() else {return};
    closure()
  }
}

#[macro_export]
macro_rules! defer {
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

#[cfg(test)]
mod tests {
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
}
