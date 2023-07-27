#[macro_export]
macro_rules! ioerrror {
  ($kind:ident, $msg:expr) => {
    std::io::Error::new(std::io::ErrorKind::$kind, $msg)
  };
  ($kind:ident, $fmt:expr, $($args:expr),+) => {
    std::io::Error::new(std::io::ErrorKind::$kind, format!($fmt, $($args),+))
  };
}

#[macro_export]
macro_rules! ioeresult {
  ($kind:ident, $msg:expr) => {
    Err(std::io::Error::new(std::io::ErrorKind::$kind, $msg))
  };
  ($kind:ident, $fmt:expr, $($args:expr),+) => {
    Err(std::io::Error::new(std::io::ErrorKind::$kind, format!($fmt, $($args),+)))
  };
  ($ok_type:ty | $kind:ident $(,$args:expr)+) => {
    {
      let res: Result<$ok_type, _> = $crate::ioeresult!($kind $(,$args)+);
      res
    }
  };
}

pub trait AttachToIoErrorExt {
  fn attach(&self, attachment: impl std::error::Error) -> std::io::Error;
}

impl AttachToIoErrorExt for std::io::Error {
  fn attach(&self, attachment: impl std::error::Error) -> std::io::Error {
    std::io::Error::new(self.kind(), format!("{}: {}", self, attachment))
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_ioerror() {
    let _ = ioerrror!(InvalidData, "hello{}", 1);
    let _ = ioeresult!(i32 | InvalidData, "hello{}", 1);
  }
}
