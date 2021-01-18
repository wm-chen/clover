
pub trait Happened<T> {
  /// The thing happened.
  fn happened(t: &T);
}

impl<T> Happened<T> for () {
  fn happened(_: &T) {}
}