pub trait ResultIgnoreExt {
  type Result;
  fn ignore(self) -> Self::Result;
}

impl<T> ResultIgnoreExt for Result<T, T> {
  type Result = T;
  fn ignore(self) -> Self::Result {
    match self {
      Ok(res) => res,
      Err(res) => res,
    }
  }
}

impl<T> ResultIgnoreExt for Result<T, std::sync::PoisonError<T>> {
  type Result = T;
  fn ignore(self) -> Self::Result {
    match self {
      Ok(result) => result,
      Err(err) => err.into_inner(),
    }
  }
}