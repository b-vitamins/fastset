use super::core::Set;
use std::collections::HashSet;

/// Performs the union operation between two references to `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = &set1 | &set2;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOr<&'a Set> for &'a Set {
    type Output = Set;

    fn bitor(self, rhs: &'a Set) -> Set {
        self.union(rhs)
    }
}

/// Performs the union operation between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset = HashSet::<usize>::from_iter(3..8);
/// let result = &set | &hashset;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOr<&'a HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitor(self, rhs: &'a HashSet<usize>) -> Set {
        self.union(rhs)
    }
}

/// Performs the union operation between an owned `Set` and a reference to `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = set1 | &set2;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl std::ops::BitOr<&Set> for Set {
    type Output = Set;

    fn bitor(self, rhs: &Set) -> Set {
        self.union(rhs)
    }
}

/// Performs the union operation between an owned `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = set | &hashset;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl std::ops::BitOr<&HashSet<usize>> for Set {
    type Output = Set;

    fn bitor(self, rhs: &HashSet<usize>) -> Set {
        self.union(rhs)
    }
}

/// Performs the union operation between a reference to `Set` and an owned `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = &set1 | set2;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOr<Set> for &'a Set {
    type Output = Set;

    fn bitor(self, rhs: Set) -> Set {
        self.union(&rhs)
    }
}

/// Performs the union operation between a reference to `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = &set | hashset;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOr<HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitor(self, rhs: HashSet<usize>) -> Set {
        self.union(&rhs)
    }
}

/// Performs the union operation between two owned `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = set1 | set2;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl std::ops::BitOr for Set {
    type Output = Set;

    fn bitor(self, rhs: Set) -> Set {
        self.union(&rhs)
    }
}

/// Performs the union operation between an owned `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = set | hashset;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl std::ops::BitOr<HashSet<usize>> for Set {
    type Output = Set;

    fn bitor(self, rhs: HashSet<usize>) -> Set {
        self.union(&rhs)
    }
}

/// Performs the union assignment operation between two references to `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let mut set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// set1 |= &set2;
/// assert_eq!(set1, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOrAssign<&'a Set> for Set {
    fn bitor_assign(&mut self, rhs: &'a Set) {
        *self = self.union(rhs);
    }
}

/// Performs the union assignment operation between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let mut set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// set |= &hashset;
/// assert_eq!(set, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOrAssign<&'a HashSet<usize>> for Set {
    fn bitor_assign(&mut self, rhs: &'a HashSet<usize>) {
        *self = self.union(rhs);
    }
}

/// Performs the intersection operation between two references to `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let intersection = &set1 & &set2;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAnd<&'a Set> for &'a Set {
    type Output = Set;

    fn bitand(self, rhs: &'a Set) -> Set {
        self.intersection(rhs)
    }
}

/// Performs the intersection operation between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let intersection = &set & &hashset;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAnd<&'a HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitand(self, rhs: &'a HashSet<usize>) -> Set {
        self.intersection(rhs)
    }
}

/// Performs the intersection operation between an owned `Set` and a reference to `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let intersection = set1 & &set2;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl std::ops::BitAnd<&Set> for Set {
    type Output = Set;

    fn bitand(self, rhs: &Set) -> Set {
        self.intersection(rhs)
    }
}

/// Performs the intersection operation between an owned `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let intersection = set & &hashset;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl std::ops::BitAnd<&HashSet<usize>> for Set {
    type Output = Set;

    fn bitand(self, rhs: &HashSet<usize>) -> Set {
        self.intersection(rhs)
    }
}

/// Performs the intersection operation between a reference to `Set` and an owned `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let intersection = &set1 & set2;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAnd<Set> for &'a Set {
    type Output = Set;

    fn bitand(self, rhs: Set) -> Set {
        self.intersection(&rhs)
    }
}

/// Performs the intersection operation between a reference to `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let intersection = &set & hashset;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAnd<HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitand(self, rhs: HashSet<usize>) -> Set {
        self.intersection(&rhs)
    }
}

/// Performs the intersection operation between two owned `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let intersection = set1 & set2;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl std::ops::BitAnd for Set {
    type Output = Set;

    fn bitand(self, rhs: Set) -> Set {
        self.intersection(&rhs)
    }
}

/// Performs the intersection operation between an owned `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let intersection = set & hashset;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl std::ops::BitAnd<HashSet<usize>> for Set {
    type Output = Set;

    fn bitand(self, rhs: HashSet<usize>) -> Set {
        self.intersection(&rhs)
    }
}

/// Performs the intersection assignment operation between two `Set` references.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let mut set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// set1 &= &set2;
/// assert_eq!(set1, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAndAssign<&'a Set> for Set {
    fn bitand_assign(&mut self, rhs: &'a Set) {
        *self = self.intersection(rhs);
    }
}

/// Performs the intersection assignment operation between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let mut set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// set &= &hashset;
/// assert_eq!(set, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAndAssign<&'a HashSet<usize>> for Set {
    fn bitand_assign(&mut self, rhs: &'a HashSet<usize>) {
        *self = self.intersection(rhs);
    }
}

/// Performs the subtraction operation between two `Set` references.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = &set1 - &set2;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::Sub<&'a Set> for &'a Set {
    type Output = Set;

    fn sub(self, rhs: &'a Set) -> Set {
        self.difference(rhs)
    }
}

/// Performs the subtraction operation between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = &set - &hashset;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::Sub<&'a HashSet<usize>> for &'a Set {
    type Output = Set;

    fn sub(self, rhs: &'a HashSet<usize>) -> Set {
        self.difference(rhs)
    }
}

/// Performs the subtraction operation between an owned `Set` and a reference to `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = set1 - &set2;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl std::ops::Sub<&Set> for Set {
    type Output = Set;

    fn sub(self, rhs: &Set) -> Set {
        self.difference(rhs)
    }
}

/// Performs the subtraction operation between an owned `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = set - &hashset;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl std::ops::Sub<&HashSet<usize>> for Set {
    type Output = Set;

    fn sub(self, rhs: &HashSet<usize>) -> Set {
        self.difference(rhs)
    }
}

/// Performs the subtraction operation between a reference to `Set` and an owned `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = &set1 - set2;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::Sub<Set> for &'a Set {
    type Output = Set;

    fn sub(self, rhs: Set) -> Set {
        self.difference(&rhs)
    }
}

/// Performs the subtraction operation between a reference to `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = &set - hashset;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::Sub<HashSet<usize>> for &'a Set {
    type Output = Set;

    fn sub(self, rhs: HashSet<usize>) -> Set {
        self.difference(&rhs)
    }
}

/// Performs the subtraction operation between two owned `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = set1 - set2;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl std::ops::Sub for Set {
    type Output = Set;

    fn sub(self, rhs: Set) -> Set {
        self.difference(&rhs)
    }
}

/// Performs the subtraction operation between an owned `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = set - hashset;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl std::ops::Sub<HashSet<usize>> for Set {
    type Output = Set;

    fn sub(self, rhs: HashSet<usize>) -> Set {
        self.difference(&rhs)
    }
}

/// Performs the subtraction assignment operation between a `Set` reference and another `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let mut set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// set1 -= &set2;
/// assert_eq!(set1, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::SubAssign<&'a Set> for Set {
    fn sub_assign(&mut self, rhs: &'a Set) {
        *self = self.difference(rhs);
    }
}

/// Performs the subtraction assignment operation between a `Set` reference and a `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let mut set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// set -= &hashset;
/// assert_eq!(set, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::SubAssign<&'a HashSet<usize>> for Set {
    fn sub_assign(&mut self, rhs: &'a HashSet<usize>) {
        *self = self.difference(rhs);
    }
}

/// Performs the subtraction assignment operation between a `Set` and another `Set` reference.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let mut set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// set1 -= set2;
/// assert_eq!(set1, Set::from_iter(0..3));
/// ```
impl std::ops::SubAssign<Set> for Set {
    fn sub_assign(&mut self, rhs: Set) {
        *self = self.difference(&rhs);
    }
}

/// Performs the subtraction assignment operation between a `Set` and a `HashSet<usize>` reference.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let mut set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// set -= hashset;
/// assert_eq!(set, Set::from_iter(0..3));
/// ```
impl std::ops::SubAssign<HashSet<usize>> for Set {
    fn sub_assign(&mut self, rhs: HashSet<usize>) {
        *self = self.difference(&rhs);
    }
}

/// Computes the symmetric difference between two `Set` references.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let symmetric_difference = &set1 ^ &set2;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXor<&'a Set> for &'a Set {
    type Output = Set;

    fn bitxor(self, rhs: &'a Set) -> Set {
        self.symmetric_difference(rhs)
    }
}

/// Computes the symmetric difference between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let symmetric_difference = &set ^ &hashset;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXor<&'a HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitxor(self, rhs: &'a HashSet<usize>) -> Set {
        self.symmetric_difference(rhs)
    }
}

/// Computes the symmetric difference between an owned `Set` and a reference to `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let symmetric_difference = set1 ^ &set2;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl std::ops::BitXor<&Set> for Set {
    type Output = Set;

    fn bitxor(self, rhs: &Set) -> Set {
        self.symmetric_difference(rhs)
    }
}

/// Computes the symmetric difference between an owned `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let symmetric_difference = set ^ &hashset;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl std::ops::BitXor<&HashSet<usize>> for Set {
    type Output = Set;

    fn bitxor(self, rhs: &HashSet<usize>) -> Set {
        self.symmetric_difference(rhs)
    }
}

/// Computes the symmetric difference between a reference to `Set` and an owned `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let symmetric_difference = &set1 ^ set2;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXor<Set> for &'a Set {
    type Output = Set;

    fn bitxor(self, rhs: Set) -> Set {
        self.symmetric_difference(&rhs)
    }
}

/// Computes the symmetric difference between a reference to `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let symmetric_difference = &set ^ hashset;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXor<HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitxor(self, rhs: HashSet<usize>) -> Set {
        self.symmetric_difference(&rhs)
    }
}

/// Computes the symmetric difference between two owned `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let symmetric_difference = set1 ^ set2;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl std::ops::BitXor for Set {
    type Output = Set;

    fn bitxor(self, rhs: Set) -> Set {
        self.symmetric_difference(&rhs)
    }
}

/// Computes the symmetric difference between an owned `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let symmetric_difference = set ^ hashset;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl std::ops::BitXor<HashSet<usize>> for Set {
    type Output = Set;

    fn bitxor(self, rhs: HashSet<usize>) -> Set {
        self.symmetric_difference(&rhs)
    }
}

/// Computes the symmetric difference between two `Set` references and assigns the result to the left operand.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let mut set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// set1 ^= &set2;
/// assert_eq!(set1, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXorAssign<&'a Set> for Set {
    fn bitxor_assign(&mut self, rhs: &'a Set) {
        *self = self.symmetric_difference(rhs);
    }
}

/// Computes the symmetric difference between a reference to `Set` and a reference to `HashSet<usize>` and assigns the result to the left operand.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let mut set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// set ^= &hashset;
/// assert_eq!(set, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXorAssign<&'a HashSet<usize>> for Set {
    fn bitxor_assign(&mut self, rhs: &'a HashSet<usize>) {
        *self = self.symmetric_difference(rhs);
    }
}
