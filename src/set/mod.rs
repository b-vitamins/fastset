mod conversions;
mod core;
mod iterators;
mod operators;
mod ops;
mod traits;

#[cfg(test)]
mod tests;

pub use self::core::Set;
pub use self::ops::SetOps;

// Re-export MAX_CAPACITY for internal use
pub(crate) use crate::MAX_CAPACITY;
