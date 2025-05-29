use super::core::Set;

/// Consumes the `Set`, returning an iterator over owned `usize` values.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let set = Set::from(vec![1, 2, 3]);
/// let mut iter = set.into_iter();
///
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), None);
/// ```
impl IntoIterator for Set {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

/// Borrows the `Set`, returning an iterator over references to `usize` values.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let set = Set::from(vec![1, 2, 3]);
///
/// for &value in &set {
///     println!("{}", value);
/// }
/// ```
impl<'a> IntoIterator for &'a Set {
    type Item = &'a usize;
    type IntoIter = std::slice::Iter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

/// Mutably borrows the `Set`, returning an iterator over mutable references to `usize` values.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let mut set = Set::from(vec![1, 2, 3]);
///
/// for value in &mut set {
///     *value += 1;
/// }
/// ```
impl<'a> IntoIterator for &'a mut Set {
    type Item = &'a mut usize;
    type IntoIter = std::slice::IterMut<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}
