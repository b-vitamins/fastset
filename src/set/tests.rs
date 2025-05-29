use super::*;
use crate::MAX_CAPACITY;
use nanorand::{Rng, WyRand};
use statrs::distribution::{ChiSquared, ContinuousCDF};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[test]
fn new_with_zero_max_element() {
    let set = Set::with_max(0);
    assert!(set.is_empty());
    assert!(set.elements.is_empty());
    assert!(!set.indicator.is_empty());
    assert_eq!(set.max, 0);
}

#[test]
fn new_with_nonzero_max_element() {
    let max_element = 10;
    let set = Set::with_max(max_element);
    assert_eq!(set.elements.len(), 0);
    assert_eq!(set.len(), 0);
    assert_eq!(set.max, max_element);
}

#[test]
fn new_with_large_max_element() {
    let max_element = 1000000;
    let set = Set::with_max(max_element);
    assert_eq!(set.elements.len(), 0);
    assert_eq!(set.max, max_element);
}

#[test]
fn new_with_multiple_calls() {
    let set1 = Set::with_max(5);
    let set2 = Set::with_max(10);

    assert_eq!(set1.max, 5);
    assert_eq!(set2.max, 10);
}

#[test]
fn with_capacity_zero() {
    let set = Set::with_capacity(0);

    // Now with proper protection, indicator will always have at least 1 element
    assert_eq!(set.indicator.len(), 1);
    assert!(set.elements.is_empty());
    assert!(set.pages.is_empty());
    assert_eq!(set.max, 0);
}

#[test]
fn with_capacity_nonzero() {
    let capacity = 10;
    let set = Set::with_capacity(capacity);

    assert_eq!(set.elements.len(), 0);
    // indicator.len() is now capacity + 1 to handle values 0..=capacity
    assert_eq!(set.indicator.len(), capacity + 1);
    // max is now equal to capacity (not capacity-1)
    assert_eq!(set.max, capacity);
}

#[test]
fn with_capacity_large() {
    let capacity = 1000000;
    let set = Set::with_capacity(capacity);

    assert_eq!(set.elements.len(), 0);
    // indicator.len() is now capacity + 1 to handle values 0..=capacity
    assert_eq!(set.indicator.len(), capacity + 1);
    // max is now equal to capacity (not capacity-1)
    assert_eq!(set.max, capacity);
}

#[test]
fn with_capacity_multiple_calls() {
    let set1 = Set::with_capacity(5);
    let set2 = Set::with_capacity(10);

    // max is now equal to capacity (not capacity-1)
    assert_eq!(set1.max, 5);
    assert_eq!(set2.max, 10);
}

#[test]
fn reserve_increase_capacity() {
    let mut set = Set::with_max(5);
    set.reserve(10);

    assert_eq!(set.max, 10);
    assert_eq!(set.indicator.len(), 11);
}

#[test]
fn reserve_no_increase_capacity() {
    let mut set = Set::with_max(5);
    set.reserve(3);

    assert_eq!(set.max, 5);
    assert_eq!(set.indicator.len(), 6);
}

#[test]
fn reserve_same_capacity() {
    let mut set = Set::with_max(5);
    set.reserve(5);

    assert_eq!(set.max, 5);
    assert_eq!(set.indicator.len(), 6);
}

#[test]
fn reserve_large_capacity() {
    let mut set = Set::with_max(5);
    set.reserve(100);

    assert_eq!(set.max, 100);
    assert_eq!(set.indicator.len(), 101);
}

#[test]
fn len_empty_set() {
    let set = Set::with_max(5);
    assert_eq!(set.len(), 0);
}

#[test]
fn len_non_empty_set() {
    let mut set = Set::with_max(5);
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert_eq!(set.len(), 3);
}

#[test]
fn is_empty_empty_set() {
    let set = Set::with_max(5);
    assert!(set.is_empty());
}

#[test]
fn is_empty_non_empty_set() {
    let mut set = Set::with_max(5);
    set.insert(1);

    assert!(!set.is_empty());
}

#[test]
fn iter_empty_set() {
    let set = Set::with_max(5);
    let mut iter = set.iter();

    assert_eq!(iter.next(), None);
}

#[test]
fn iter_non_empty_set() {
    let mut set = Set::with_max(5);
    set.insert(1);
    set.insert(2);

    let mut iter = set.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
}

#[test]
fn clear() {
    let mut set = Set::with_max(3);
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.clear();
    assert!(set.is_empty());
    for i in 1..=3 {
        assert!(!set.contains(&i));
    }
}

#[test]
fn clear_empty_set() {
    let mut set = Set::with_max(5);
    set.clear();

    assert!(set.is_empty());
}

#[test]
fn clear_non_empty_set() {
    let mut set = Set::with_max(5);
    set.insert(1);
    set.insert(2);
    set.clear();

    assert!(set.is_empty());
}

#[test]
fn insert() {
    let mut set = Set::with_max(MAX_CAPACITY);
    // Insert a value and check its presence
    set.insert(1);
    assert!(set.contains(&1), "Set should contain 1");
    // Insert duplicate value and check set length
    set.insert(1);
    assert_eq!(
        set.len(),
        1,
        "Inserting a duplicate should not increase set size"
    );
    // Insert more values, including one beyond the initial capacity
    set.insert(2);
    set.insert(1000);
    assert!(set.contains(&2), "Set should contain 2");
    assert!(set.contains(&1000), "Set should contain 1000");
    assert_eq!(set.len(), 3, "Set should contain 3 unique elements");
}

#[test]
fn insert_within_capacity() {
    let mut set = Set::with_max(5);
    assert!(set.insert(1));
    assert!(set.contains(&1));
    assert_eq!(set.len(), 1);
}

#[test]
fn insert_beyond_capacity_and_within_max() {
    let mut set = Set::with_max(5);
    assert!(set.insert(6));
    assert!(set.contains(&6));
    assert_eq!(set.len(), 1);
}

#[test]
#[should_panic]
fn insert_beyond_max_capacity() {
    let mut set = Set::with_max(usize::MAX);
    assert!(!set.insert(usize::MAX));
    assert!(!set.contains(&usize::MAX));
    assert_eq!(set.len(), 0);
}

#[test]
fn remove() {
    let mut set = Set::with_max(MAX_CAPACITY / 3000);
    // Insert some values
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    // Remove a value that exists
    set.remove(&3);
    assert!(!set.contains(&3), "Set should not contain 3 after removal");
    // Remove a value that does not exist
    set.remove(&5);
    assert!(
        !set.contains(&5),
        "Set should not contain 5 as it was never added"
    );
    // Check the remaining values
    assert!(set.contains(&1), "Set should still contain 1");
    assert!(set.contains(&2), "Set should still contain 2");
    assert!(set.contains(&4), "Set should still contain 4");
    // Check the total number of elements after removals
    assert_eq!(set.len(), 3, "Set should contain 3 elements after removals");
    // Remove all remaining values
    set.remove(&1);
    set.remove(&2);
    set.remove(&4);
    assert!(
        set.is_empty(),
        "Set should be empty after removing all elements"
    );
}

#[test]
fn remove_existing_element() {
    let mut set = Set::with_max(5);
    set.insert(3);
    assert!(set.remove(&3));
    assert!(!set.contains(&3));
    assert_eq!(set.len(), 0);
}

#[test]
fn remove_non_existing_element() {
    let mut set = Set::with_max(5);
    set.insert(3);
    assert!(!set.remove(&5));
    assert_eq!(set.len(), 1);
}

#[test]
fn remove_element_beyond_capacity() {
    let mut set = Set::with_max(5);
    set.insert(3);
    assert!(!set.remove(&(usize::MAX)));
    assert_eq!(set.len(), 1);
}

#[test]
fn contains() {
    let mut set = Set::with_max(MAX_CAPACITY);
    // Insert some values
    set.insert(1);
    set.insert(2);
    set.insert(3);
    // Check for presence of inserted values
    assert!(set.contains(&1), "Set should contain 1");
    assert!(set.contains(&2), "Set should contain 2");
    assert!(set.contains(&3), "Set should contain 3");
    // Check for absence of values not inserted
    assert!(!set.contains(&4), "Set should not contain 4");
    assert!(!set.contains(&0), "Set should not contain 0");
    assert!(!set.contains(&100), "Set should not contain 100");
}

#[test]
fn contains_existing_element() {
    let set = Set::from(vec![1, 2, 3]);
    assert!(set.contains(&2));
}

#[test]
fn contains_non_existing_element() {
    let set = Set::from(vec![1, 2, 3]);
    assert!(!set.contains(&5));
}

#[test]
fn contains_element_beyond_capacity() {
    let set = Set::with_max(5);
    assert!(!set.contains(&(usize::MAX)));
}

#[test]
fn get_existing_element() {
    let set = Set::from(vec![1, 2, 3]);
    assert_eq!(set.get(&2), Some(2));
}

#[test]
fn get_non_existing_element() {
    let set = Set::from(vec![1, 2, 3]);
    assert_eq!(set.get(&5), None);
}

#[test]
fn get_element_beyond_capacity() {
    let set = Set::with_max(5);
    assert_eq!(set.get(&(usize::MAX)), None);
}

#[test]
fn take_existing_element() {
    let mut set = Set::from(vec![1, 2, 3]);
    assert_eq!(set.take(&2), Some(2));
    assert!(!set.contains(&2));
}

#[test]
fn take_non_existing_element() {
    let mut set = Set::from(vec![1, 2, 3]);
    assert_eq!(set.take(&5), None);
    assert_eq!(set.len(), 3);
}

#[test]
fn take_element_beyond_capacity() {
    let mut set = Set::with_max(5);
    assert_eq!(set.take(&(usize::MAX)), None);
}

#[test]
fn max_empty_set() {
    let set = Set::with_max(5);
    assert_eq!(set.max(), None);
}

#[test]
fn max_non_empty_set() {
    let set = Set::from(vec![1, 2, 3]);
    assert_eq!(set.max(), Some(3));
}

#[test]
fn max_set_with_single_element() {
    let set = Set::from(vec![5]);
    assert_eq!(set.max(), Some(5));
}

#[test]
fn min_empty_set() {
    let set = Set::with_max(5);
    assert_eq!(set.min(), None);
}

#[test]
fn min_non_empty_set() {
    let set = Set::from(vec![3, 1, 5]);
    assert_eq!(set.min(), Some(1));
}

#[test]
fn min_set_with_single_element() {
    let set = Set::from(vec![5]);
    assert_eq!(set.min(), Some(5));
}

#[test]
fn range_cardinality_empty_set() {
    let set = Set::with_max(10);
    assert_eq!(set.range_cardinality(..), 0);
    assert_eq!(set.range_cardinality(0..5), 0);
}

#[test]
fn range_cardinality_full_set() {
    let set = Set::from(vec![1, 2, 3, 4, 5]);
    assert_eq!(set.range_cardinality(..), 5);
    assert_eq!(set.range_cardinality(1..4), 3);
}

#[test]
fn range_cardinality_out_of_bounds() {
    let set = Set::from(vec![1, 2, 3, 4, 5]);
    assert_eq!(set.range_cardinality(6..10), 0);
}

#[test]
fn rank_empty_set() {
    let set = Set::with_max(10);
    assert_eq!(set.rank(5), 0);
}

#[test]
fn rank_non_empty_set() {
    let set = Set::from(vec![1, 3, 5, 7, 9]);
    assert_eq!(set.rank(5), 2);
}

#[test]
fn rank_non_existing_element() {
    let set = Set::from(vec![1, 3, 5, 7, 9]);
    assert_eq!(set.rank(6), 3);
}

#[test]
fn remove_largest_from_empty_set() {
    let mut set = Set::with_max(10);
    assert_eq!(set.remove_largest(), None);
}

#[test]
fn remove_largest_from_non_empty_set() {
    let mut set = Set::from(vec![1, 3, 5, 7, 9]);
    assert_eq!(set.remove_largest(), Some(9));
    assert!(!set.contains(&9));
}

#[test]
fn remove_largest_from_unsorted_set() {
    let mut set = Set::with_max(100);
    set.insert(10);
    set.insert(1);
    set.insert(7);
    assert_eq!(set.remove_largest(), Some(10));
    assert!(!set.contains(&10));
}

#[test]
fn remove_smallest_from_empty_set() {
    let mut set = Set::with_max(10);
    assert_eq!(set.remove_smallest(), None);
}

#[test]
fn remove_smallest_from_non_empty_set() {
    let mut set = Set::from(vec![1, 3, 5, 7, 9]);
    assert_eq!(set.remove_smallest(), Some(1));
    assert!(!set.contains(&1));
}

#[test]
fn remove_smallest_from_unsorted_set() {
    let mut set = Set::with_max(100);
    set.insert(10);
    set.insert(1);
    set.insert(7);
    assert_eq!(set.remove_smallest(), Some(1));
    assert!(!set.contains(&1));
}

#[test]
fn random() {
    let mut set = Set::with_max(MAX_CAPACITY / 3000);
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    // Test with non-empty set
    let mut observed_values = HashSet::new();
    let mut rng = WyRand::new();
    for _ in 0..100 {
        if let Some(value) = set.random(&mut rng) {
            assert!(
                set.contains(&value),
                "Randomly selected value should be in the set"
            );
            observed_values.insert(value);
        }
    }
    // Check that multiple distinct values are observed
    assert!(
        observed_values.len() > 1,
        "Random should return different values over multiple calls"
    );
    // Test with empty set
    set.clear();
    assert!(
        set.random(&mut rng).is_none(),
        "Random should return None for an empty set"
    );
}

#[test]
fn random_returns_none_for_empty_set() {
    let set = Set::with_max(10);
    let mut rng = WyRand::new();
    assert_eq!(set.random(&mut rng), None);
}

#[test]
fn insert_unchecked_adds_element_correctly() {
    let mut set = Set::with_max(5);

    // Insert an element without bounds checking
    let result = set.insert_unchecked(3);

    // Ensure the element was inserted and the operation returned true
    assert!(result);
    assert_eq!(set.len(), 1);
    assert!(set.contains(&3));
    assert_eq!(set.current_max, Some(3));
    assert_eq!(set.current_min, Some(3));
}

#[test]
fn remove_unchecked_removes_element_correctly() {
    let mut set = Set::with_max(5);
    set.insert(3);

    // Remove the element without bounds checking
    let result = unsafe { set.remove_unchecked(&3) };

    // Ensure the element was removed and the operation returned true
    assert!(result);
    assert_eq!(set.len(), 0);
    assert!(!set.contains(&3));
    assert_eq!(set.current_max, None);
    assert_eq!(set.current_min, None);
}

#[test]
#[should_panic]
fn remove_unchecked_panics_for_out_of_bounds() {
    let mut set = Set::with_max(5);
    set.insert(3);
    // Attempt to remove an out-of-bounds element without bounds checking should panic
    unsafe { assert!(!set.remove_unchecked(&6)) };
}

#[test]
fn contains_returns_true_for_existing_element() {
    let mut set = Set::with_max(100);
    set.insert(42);

    // Check if the set contains the inserted value
    assert!(set.contains(&42));
}

#[test]
fn contains_returns_false_for_nonexistent_element() {
    let set = HashSet::<usize>::new();
    // Check if the set contains a value not inserted
    assert!(!set.contains(&100));
}

#[test]
fn iter_returns_correct_values() {
    let mut set = Set::with_max(2);
    set.insert(42);
    set.insert(100);
    // Create an iterator from the set
    let mut iter = set.iter();
    // Check the values returned by the iterator
    assert_eq!(iter.next(), Some(&42));
    assert_eq!(iter.next(), Some(&100));
    assert_eq!(iter.next(), None);
}

#[test]
fn max_returns_correct_value() {
    let mut set = HashSet::new();
    set.insert(42);
    set.insert(100);
    // Check if the maximum value in the set is returned
    assert_eq!(set.max(), Some(100));
}

#[test]
fn max_returns_none_for_empty_set() {
    let set = HashSet::new();

    // Check if None is returned for an empty set
    assert_eq!(set.max(), None);
}

#[test]
fn is_subset_returns_true_for_subset() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(1..=10);

    // Check if set1 is a subset of set2
    assert!(set1.is_subset(&set2));
}

#[test]
fn is_subset_returns_false_for_non_subset() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(6..=10);

    // Check if set1 is a subset of set2
    assert!(!set1.is_subset(&set2));
}

#[test]
fn is_superset_returns_true_for_superset() {
    let set1 = Set::from_iter(1..=10);
    let set2 = Set::from_iter(1..=5);

    // Check if set1 is a superset of set2
    assert!(set1.is_superset(&set2));
}

#[test]
fn is_superset_returns_false_for_non_superset() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(6..=10);

    // Check if set1 is a superset of set2
    assert!(!set1.is_superset(&set2));
}

#[test]
fn is_disjoint_returns_true_for_disjoint_sets() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(6..=10);

    // Check if set1 and set2 are disjoint
    assert!(set1.is_disjoint(&set2));
}

#[test]
fn is_disjoint_returns_false_for_non_disjoint_sets() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=10);

    // Check if set1 and set2 are disjoint
    assert!(!set1.is_disjoint(&set2));
}

#[test]
fn test_intersection() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    let intersection = set1.intersection(&set2);

    assert_eq!(intersection.len(), 2);
    for i in 4..=5 {
        assert!(intersection.contains(&i));
    }
}

#[test]
fn test_difference() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    let difference = set1.difference(&set2);

    assert_eq!(difference.len(), 3);
    for i in 1..=3 {
        assert!(difference.contains(&i));
    }
}

#[test]
fn test_symmetric_difference() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    let symmetric_difference = set1.symmetric_difference(&set2);

    assert_eq!(symmetric_difference.len(), 6);
    for i in 1..=3 {
        assert!(symmetric_difference.contains(&i));
    }
    for i in 6..=8 {
        assert!(symmetric_difference.contains(&i));
    }
}

#[test]
fn test_empty_set_operations() {
    let set1 = Set::with_max(100);
    let set2 = Set::with_max(100);

    assert!(set1.union(&set2).is_empty());
    assert!(set1.intersection(&set2).is_empty());
    assert!(set1.difference(&set2).is_empty());
    assert!(set1.symmetric_difference(&set2).is_empty());
}

#[test]
fn test_sets_with_same_elements() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(1..=5);

    assert_eq!(set1.union(&set2), set1.intersection(&set2));
    assert!(set1.difference(&set2).is_empty());
    assert!(set1.symmetric_difference(&set2).is_empty());
}

#[test]
fn test_boundary_cases() {
    // Test when one set is empty
    let set1 = Set::with_max(10);
    let set2 = Set::from_iter(1..=5);

    assert!(set1.is_subset(&set2));
    assert!(!set2.is_subset(&set1));
    assert!(!set1.is_superset(&set2));
    assert!(set2.is_superset(&set1));
    assert!(set1.is_disjoint(&set2));

    // Test when one set is a subset/superset of the other
    let set3 = Set::from_iter(1..=5);
    let set4 = Set::from_iter(1..=10);

    assert!(set3.is_subset(&set4));
    assert!(!set4.is_subset(&set3));
    assert!(!set3.is_superset(&set4));
    assert!(set4.is_superset(&set3));
    assert!(!set3.is_disjoint(&set4));

    // Test when the sets have only one common element
    let set5 = Set::from_iter(1..=5);
    let set6 = Set::from_iter(4..=8);

    assert!(!set5.is_subset(&set6));
    assert!(!set6.is_subset(&set5));
    assert!(!set5.is_superset(&set6));
    assert!(!set6.is_superset(&set5));
    assert!(!set5.is_disjoint(&set6));
}

#[test]
fn test_bit_xor_sets() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    let result = &set1 ^ &set2;

    // Verify that the result contains only the elements that are unique to each set
    assert_eq!(result.len(), 6);
    assert!(result.contains(&6));
    assert!(result.contains(&7));
    assert!(result.contains(&8));
}

#[test]
fn test_bit_xor_assignment_sets() {
    let mut set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    set1 ^= &set2;

    // Verify that set1 contains only the elements that are unique to each set
    assert_eq!(set1.len(), 6);
    assert!(set1.contains(&1));
    assert!(set1.contains(&2));
    assert!(set1.contains(&3));
    assert!(!set1.contains(&4));
    assert!(!set1.contains(&5));
    assert!(set1.contains(&6));
    assert!(set1.contains(&7));
    assert!(set1.contains(&8));
}

#[test]
fn test_bit_xor_assignment_set_and_hashset() {
    let mut set = Set::from_iter(1..=5);
    let hash_set = (4..=8).collect::<HashSet<_>>();

    set ^= &hash_set;

    // Verify that the set contains only the elements that are unique to each set
    assert_eq!(set.len(), 6);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(set.contains(&6));
    assert!(set.contains(&7));
    assert!(set.contains(&8));
}

#[test]
fn test_sub_sets() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    let result = &set1 - &set2;

    // Verify that the result contains only the elements present in set1 but not in set2
    assert_eq!(result.len(), 3);
    assert!(result.contains(&1));
    assert!(result.contains(&2));
    assert!(result.contains(&3));
}

#[test]
fn test_sub_set_and_hashset() {
    let set = Set::from_iter(1..=5);
    let hash_set = (4..=8).collect::<HashSet<_>>();

    let result = &set - &hash_set;

    // Verify that the result contains only the elements present in the set but not in the hash set
    assert_eq!(result.len(), 3);
    assert!(result.contains(&1));
    assert!(result.contains(&2));
    assert!(result.contains(&3));
    assert!(!result.contains(&4));
}

#[test]
fn test_sub_assignment_sets() {
    let mut set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    set1 -= &set2;

    // Verify that set1 contains only the elements present in set1 but not in set2
    assert_eq!(set1.len(), 3);
    assert!(set1.contains(&1));
    assert!(set1.contains(&2));
    assert!(set1.contains(&3));
}

#[test]
fn test_sub_assignment_set_and_hashset() {
    let mut set = Set::from_iter(1..=5);
    let hash_set = (4..=8).collect::<HashSet<_>>();

    set -= &hash_set;

    // Verify that the set contains only the elements present in the set but not in the hash set
    assert_eq!(set.len(), 3);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
}

#[test]
fn test_bitand_sets() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    let result = &set1 & &set2;

    // Verify that the result contains only the elements present in both set1 and set2
    assert_eq!(result.len(), 2);
    assert!(result.contains(&4));
    assert!(result.contains(&5));
}

#[test]
fn test_bitand_set_and_hashset() {
    let set = Set::from_iter(1..=5);
    let hash_set = (4..=8).collect::<HashSet<_>>();

    let result = &set & &hash_set;

    // Verify that the result contains only the elements present in both the set and the hash set
    assert_eq!(result.len(), 2);
    assert!(result.contains(&4));
    assert!(result.contains(&5));
    assert!(!result.contains(&3));
    assert!(!result.contains(&7));
}

#[test]
fn test_bitand_assignment_sets() {
    let mut set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    set1 &= &set2;

    // Verify that set1 contains only the elements present in both set1 and set2
    assert_eq!(set1.len(), 2);
    assert!(set1.contains(&4));
    assert!(set1.contains(&5));
}

#[test]
fn test_bitand_assignment_set_and_hashset() {
    let mut set = Set::from_iter(1..=5);
    let hash_set = (4..=8).collect::<HashSet<_>>();

    set &= &hash_set;

    assert_eq!(set.len(), 2);
    assert!(set.contains(&4));
    assert!(set.contains(&5));

    let mut set2 = Set::from_iter(1..5);
    let hash_set2 = (4..=8).collect::<HashSet<_>>();

    set2 &= &hash_set2;

    assert_eq!(set2.len(), 1);
    assert!(set2.contains(&4));
    assert!(!set2.contains(&5));

    let mut set3 = Set::from_iter(1..5);
    let hash_set3 = (6..=8).collect::<HashSet<_>>();

    set3 &= &hash_set3;

    assert_eq!(set3.len(), 0);
    assert!(!set3.contains(&1));
}

#[test]
fn debug_format() {
    let mut set = Set::with_max(5); // Assuming a 'with_max' method with a 'max' parameter.

    // Simulate `from_iter` functionality for the test.
    for i in 1..=5 {
        set.insert(i); // Assuming an 'insert' method is available.
    }

    let debug_output = format!("{:?}", set);

    assert!(debug_output.contains("Set {"));
    assert!(debug_output.contains("elements: [1, 2, 3, 4, 5]"));
    assert!(debug_output.contains("max: 5"));
    assert!(debug_output.contains("current_max: Some(5)"));
    assert!(debug_output.contains("current_min: Some(1)"));

    // Adjust the assertions to match the new Debug output format:
    // We're looking for "Mapped Index:" now instead of "Index: Some()"
    // Example new format: "Element: 1, Indicator: true, Mapped Index: 0"
    // Note: The exact wording and ordering in your Debug impl may require adjustments here.
    assert!(debug_output.contains("Element: 1, Indicator: true, Mapped Index: 0"));
    assert!(debug_output.contains("Element: 5, Indicator: true, Mapped Index: 4"));
}

#[test]
fn display_format() {
    let set = Set::from_iter(1..=3);
    let display_output = format!("{}", set);
    assert!(display_output == "{1, 2, 3}" || display_output == "{3, 2, 1}");
}

#[test]
fn test_default() {
    let set: Set = Default::default();
    assert!(set.is_empty());
}

#[test]
fn test_partial_eq_sets_equal() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(1..=5);
    assert_eq!(set1, set2);
}

#[test]
fn test_partial_eq_sets_not_equal() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(6..=10);
    assert_ne!(set1, set2);
}

#[test]
fn test_eq_sets_equal() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter((1..=5).rev());
    assert_eq!(set1, set2);
}

#[test]
fn test_eq_sets_not_equal() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(1..=4);
    assert_ne!(set1, set2);
}

#[test]
fn test_partial_eq_with_hashset() {
    let set = Set::from_iter(1..=5);
    let hash_set: HashSet<usize> = (1..=5).collect();
    assert_eq!(set, hash_set);
}

#[test]
fn test_hash() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(1..=5);

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    set1.hash(&mut hasher1);
    set2.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());
}

#[test]
fn test_from_vec() {
    let vec = vec![1, 2, 3, 4, 5];
    let set = Set::from(vec.clone());

    for item in vec {
        assert!(set.contains(&item));
    }
}

#[test]
fn test_from_slice() {
    let items = &[1, 2, 3, 4, 5];
    let set = Set::from(items);

    for &item in items {
        assert!(set.contains(&item));
    }
}

#[test]
fn test_from_array() {
    let items = &[1, 2, 3, 4, 5];
    let set = Set::from(items);

    for &item in items {
        assert!(set.contains(&item));
    }
}

#[test]
fn test_from_hashset_owned() {
    let mut hash_set = HashSet::new();
    hash_set.insert(1);
    hash_set.insert(2);
    hash_set.insert(3);

    let set = Set::from(hash_set.clone());

    for item in hash_set {
        assert!(set.contains(&item));
    }
}

#[test]
fn test_from_hashset_ref() {
    let mut hash_set = HashSet::new();
    hash_set.insert(1);
    hash_set.insert(2);
    hash_set.insert(3);

    let set = Set::from(&hash_set);

    for &item in &hash_set {
        assert!(set.contains(&item));
    }
}

#[test]
fn test_extend_usize() {
    let mut set = Set::with_max(0);
    set.extend(vec![1, 2, 3]);

    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
}

#[test]
fn test_extend_ref_usize() {
    let mut set = Set::with_max(0);
    let values = [1, 2, 3];
    set.extend(values.iter());

    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
}

#[test]
fn test_from_iterator_usize() {
    let set: Set = (1..=5).collect();

    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(set.contains(&4));
    assert!(set.contains(&5));
}

#[test]
fn test_from_iterator_ref_usize() {
    let values = [1, 2, 3];
    let set: Set = values.iter().collect();

    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
}

#[test]
fn test_into_iter_owned() {
    let set = Set::from(vec![1, 2, 3]);
    let mut iter = set.into_iter();

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_into_iter_ref() {
    let set = Set::from(vec![1, 2, 3]);

    let mut values = Vec::new();
    for &value in &set {
        values.push(value);
    }

    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_into_iter_mut_ref() {
    let mut set = Set::from(vec![1, 2, 3]);

    for value in &mut set {
        *value += 1;
    }

    assert_eq!(set.elements, vec![2, 3, 4]);
}

#[test]
fn comparison() {
    let mut set = Set::with_capacity(1_000_000);
    let mut std_set = HashSet::new();
    let mut rng = WyRand::new();
    for iteration in 0..10000 {
        let value = rng.generate_range(..100_000usize);
        // Randomly choose to insert or remove
        if rng.generate::<bool>() {
            set.insert(value);
            std_set.insert(value);
            if rng.generate::<bool>() {
                let more_values: Vec<usize> =
                    (0..10).map(|_| rng.generate_range(..1000usize)).collect();
                for &val in more_values.iter() {
                    std_set.insert(val);
                }
                for &val in more_values.iter() {
                    set.insert(val);
                }
            }
        } else {
            set.remove(&value);
            std_set.remove(&value);
            if rng.generate::<bool>() {
                let more_values: Vec<usize> =
                    (0..10).map(|_| rng.generate_range(..1000usize)).collect();
                for &val in more_values.iter() {
                    std_set.remove(&val);
                }
                for &val in more_values.iter() {
                    set.remove(&val);
                }
            }
        }
        // Periodically verify that both sets contain the same elements
        if iteration % 100 == 0 {
            let diff = set.difference(&std_set);
            if !diff.is_empty() {
                println!("Differences at iteration {}: {:?}", iteration, diff);
            }
            assert!(
                diff.is_empty(),
                "Iteration {}: HashSet and StdHashSet differ: {:?}",
                iteration,
                diff
            );
        }
    }
}

#[test]
fn test_max_element_reached() {
    // Create sets with maximum element reached
    let max_element = MAX_CAPACITY / 3000 - 1; // Assuming MAX_CAPACITY is defined somewhere
    let set1 = Set::from_iter(0..max_element);
    let set2 = Set::from_iter((max_element - 4)..=max_element);

    // Perform set operations
    let union = set1.union(&set2);
    let intersection = set1.intersection(&set2);
    let difference = set1.difference(&set2);
    let symmetric_difference = set1.symmetric_difference(&set2);

    // Verify the results
    assert_eq!(union.len(), MAX_CAPACITY / 3000); // set1 (0 to max_element-1) plus max_element from set2
    assert_eq!(intersection.len(), 4); // The elements: max-4, max-3, max-2, max-1
    assert_eq!(difference.len(), max_element - 4); // All in set1 minus intersection

    // Symmetric difference has:
    // - All elements unique to set1 (max_element - 4 elements)
    // - Plus one element unique to set2 (max_element itself)
    assert_eq!(symmetric_difference.len(), (max_element - 4) + 1);
}

#[test]
fn test_randomized_operations() {
    // Generate random sets
    let mut rng = WyRand::new();
    let set1: Set = (0..100).map(|_| rng.generate_range(0..100)).collect();
    let set2: Set = (0..150).map(|_| rng.generate_range(0..150)).collect();

    // Perform set operations
    let union = set1.union(&set2);
    let intersection = set1.intersection(&set2);
    let difference = set1.difference(&set2);
    let symmetric_difference = set1.symmetric_difference(&set2);
    // Validate the results by comparing with manual calculations
    for element in &union {
        assert!(set1.contains(element) || set2.contains(element));
    }
    for element in &intersection {
        assert!(set1.contains(element) && set2.contains(element));
    }
    for element in &difference {
        assert!(set1.contains(element) && !set2.contains(element));
    }
    for element in &symmetric_difference {
        assert!(set1.contains(element) != set2.contains(element));
    }
}

#[test]
fn test_bit_or_sets() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    let result = &set1 | &set2;

    // Verify that the result contains all elements from both sets
    assert_eq!(result.len(), 8);
    assert!(result.contains(&1));
    assert!(result.contains(&2));
    assert!(result.contains(&3));
    assert!(result.contains(&4));
    assert!(result.contains(&5));
    assert!(result.contains(&6));
    assert!(result.contains(&7));
    assert!(result.contains(&8));
}

#[test]
fn test_bit_or_set_and_hashset() {
    let set = Set::from_iter(1..=5);
    let hash_set = (4..=8).collect::<HashSet<_>>();

    let result = &set | &hash_set;

    // Verify that the result contains all elements from both sets
    assert_eq!(result.len(), 8);
    assert!(result.contains(&1));
    assert!(result.contains(&2));
    assert!(result.contains(&3));
    assert!(result.contains(&4));
    assert!(result.contains(&5));
    assert!(result.contains(&6));
    assert!(result.contains(&7));
    assert!(result.contains(&8));
}

#[test]
fn test_bit_or_assignment_sets() {
    let mut set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    set1 |= &set2;

    // Verify that set1 contains all elements from both sets
    assert_eq!(set1.len(), 8);
    assert!(set1.contains(&1));
    assert!(set1.contains(&2));
    assert!(set1.contains(&3));
    assert!(set1.contains(&4));
    assert!(set1.contains(&5));
    assert!(set1.contains(&6));
    assert!(set1.contains(&7));
    assert!(set1.contains(&8));
}

#[test]
fn test_bit_or_assignment_set_and_hashset() {
    let mut set = Set::from_iter(1..=5);
    let hash_set = (4..=8).collect::<HashSet<_>>();
    set |= &hash_set;

    // Verify that set contains all elements from both sets
    assert_eq!(set.len(), 8);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(set.contains(&4));
    assert!(set.contains(&5));
    assert!(set.contains(&6));
    assert!(set.contains(&7));
    assert!(set.contains(&8));
}

#[test]
fn test_overlapping_ranges() {
    // Create two sets with overlapping ranges
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    // Perform set operations
    let union = set1.union(&set2);
    let intersection = set1.intersection(&set2);
    let difference = set1.difference(&set2);
    let symmetric_difference = set1.symmetric_difference(&set2);

    // Verify the results
    assert_eq!(union.len(), 8);
    assert_eq!(intersection.len(), 2); // 4 and 5 are the overlapping elements
    assert_eq!(difference.len(), 3); // 1, 2, and 3 are unique to set1
    assert_eq!(symmetric_difference.len(), 6); // 1, 2, 3, 6, 7, and 8 are unique to their respective sets
}

#[test]
fn test_nested_set_operations() {
    // Create sets
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);
    let set3 = Set::from_iter(6..=10);

    // Perform set operations
    let union = set1.union(&set2);
    let nested_intersection = union.intersection(&set3);

    // Verify the result
    assert_eq!(nested_intersection.len(), 3); // 6, 7, 8 are the common elements
}

#[test]
fn test_union() {
    let set1 = Set::from_iter(1..=5);
    let set2 = Set::from_iter(4..=8);

    let union = set1.union(&set2);

    assert_eq!(union.len(), 8);
    for i in 1..=8 {
        assert!(union.contains(&i));
    }
}

#[test]
fn test_current_max_min_tracking() {
    let mut set = Set::with_max(100);

    // Test empty set
    assert_eq!(set.current_max, None);
    assert_eq!(set.current_min, None);

    // Insert elements and verify max/min update
    set.insert(10);
    assert_eq!(set.current_max, Some(10));
    assert_eq!(set.current_min, Some(10));

    set.insert(5);
    assert_eq!(set.current_max, Some(10));
    assert_eq!(set.current_min, Some(5));

    set.insert(20);
    assert_eq!(set.current_max, Some(20));
    assert_eq!(set.current_min, Some(5));

    // Remove maximum element and verify max updates
    set.remove(&20);
    assert_eq!(set.current_max, Some(10));
    assert_eq!(set.current_min, Some(5));

    // Remove minimum element and verify min updates
    set.remove(&5);
    assert_eq!(set.current_max, Some(10));
    assert_eq!(set.current_min, Some(10));

    // Remove last element
    set.remove(&10);
    assert_eq!(set.current_max, None);
    assert_eq!(set.current_min, None);
}

#[test]
fn test_remove_largest_smallest() {
    let mut set = Set::with_max(100);

    // Test with unsorted insertion
    set.insert(30);
    set.insert(10);
    set.insert(50);
    set.insert(20);
    set.insert(40);

    // Verify max/min are correct
    assert_eq!(set.current_max, Some(50));
    assert_eq!(set.current_min, Some(10));

    // Remove largest returns the largest element
    assert_eq!(set.remove_largest(), Some(50));
    assert_eq!(set.current_max, Some(40));

    // Remove smallest returns the smallest element
    assert_eq!(set.remove_smallest(), Some(10));
    assert_eq!(set.current_min, Some(20));
}

#[test]
fn sampling_is_uniformly_at_random() {
    const SAMPLES: usize = 1_000_000;
    const EDGE_OF_THE_UNIVERSE: usize = 10000;

    let elements = (1..=EDGE_OF_THE_UNIVERSE).collect::<Vec<_>>();
    let set = Set::from(elements.clone());
    let mut rng = WyRand::new_seed(42u64);
    let mut counts = vec![0f64; elements.len()];

    for _ in 0..SAMPLES {
        if let Some(value) = set.random(&mut rng) {
            counts[value - 1] += 1.0;
        }
    }

    let e = SAMPLES as f64 / elements.len() as f64;
    let statistic: f64 = counts.iter().map(|&o| (o - e) * (o - e) / e).sum();

    let dof = elements.len() - 1;
    let chi = ChiSquared::new(dof as f64).unwrap();
    let acceptable = chi.inverse_cdf(0.99);

    // Null hypothesis: Elements are sampled uniformly at random
    assert!(
        statistic < acceptable,
        "Chi-square statistic {} is greater than what's acceptable ({})",
        statistic,
        acceptable,
    );
}
