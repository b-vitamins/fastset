mod set;
pub use set::{Set, SetOps};
/// The maximum capacity for the Set.
///
/// CAUTION: Setting the set's largest element or capacity near MAX_CAPACITY
/// can significantly impact memory usage. For instance, with MAX_CAPACITY
/// set to 1 billion, the `Set` could require approximately 16 GB of memory
/// under certain conditions due to storage of elements, their indicators,
/// and indexing structures. Ensure adequate memory is available when
/// approaching this limit.
const MAX_CAPACITY: usize = 1_000_000_000;

/// Macro for creating a `Set` with the given elements.
///
/// # Example
///
/// ```
/// use fastset::{Set, set};
/// let set = set![1, 2, 3];
/// ```
#[macro_export]
macro_rules! set {
    ($($element:expr),*) => {{
        let mut new_set = Set::new(30000); // Adjusted for crate-level visibility
        $(new_set.insert($element);)*
        new_set
    }};
}

/// Macro for removing elements from a `Set`.
///
/// # Example
///
/// ```
/// # use fastset::{Set, remove};
/// let mut my_set = Set::new(10);
/// my_set.insert(1);
/// my_set.insert(2);
///
/// remove!(my_set, 1, 2);
/// ```
#[macro_export]
macro_rules! remove {
    ($set:expr, $($element:expr),*) => {
        $( $set.remove(&$element); )*
    };
}

/// Macro for inserting elements into a `Set`.
///
/// # Example
///
/// ```
/// # use fastset::{Set, insert};
/// let mut my_set = Set::new(10);
///
/// insert!(my_set, 1, 2, 3);
/// ```
#[macro_export]
macro_rules! insert {
    ($set:expr, $($element:expr),*) => {
        $( $set.insert($element); )*
    };
}

/// Macro for selecting a random element from a `Set`.
///
/// If no random number generator is provided, it will use `WyRand` as the default.
///
/// # Example
///
/// ```
/// # use fastset::{Set, random};
/// # use nanorand::{WyRand, Rng};
/// let mut my_set = Set::new(10);
/// my_set.insert(1);
/// my_set.insert(2);
///
/// let random_elem = random!(my_set); // Use default RNG
///
/// let mut rng = WyRand::new();
/// let random_elem_custom_rng = random!(my_set, &mut rng); // Use custom RNG
/// ```
#[macro_export]
macro_rules! random {
    ($set:expr, $rng:expr) => {{
        $set.random($rng)
    }};
    ($set:expr) => {{
        let mut rng = WyRand::new();
        $set.random(&mut rng)
    }};
}
