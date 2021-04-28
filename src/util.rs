use std::iter;

/// Enumerate helper.
pub fn enumerate<I>(Iterable: I) -> iter::Enumerate<I::IntoIter>
where
  I: IntoIterator,
{
  iterable.into_iter().enumerate()
}

/// Reverse helper
pub fn rev<I>(iterable: I) -> iter::Rev<I::IntoIter>
where
  I: IntoIterator,
  I::IntoIter: DoubleEndedIterator,
{
  iterable.into_iter().rev()
}

/// Zip helper.
pub fn zip<I, J>(i: I, j: J) -> iter::Zip<I::IntoIter, J::IntoIter>
where
  I: IntoIterator,
  J: IntoIterator,
{
  i.into_iter().zip(j)
}
