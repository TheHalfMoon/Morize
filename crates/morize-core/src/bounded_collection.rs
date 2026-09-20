//! Dependency-free bounded ordered collection semantics.
//!
//! This module enforces item-count ceilings only. It does not define
//! field-specific limits, serialization, persistence, sorting, uniqueness,
//! or domain-specific collection policy.

use core::fmt;

/// Bounded metadata describing why an ordered collection operation was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CollectionBoundsError {
    /// A complete input collection contains more items than permitted.
    TooManyItems {
        /// Maximum accepted number of items.
        max_items: usize,
        /// Actual number of items in the rejected collection.
        actual_items: usize,
    },
    /// An additional item cannot be inserted because the collection is full.
    AtCapacity {
        /// Maximum accepted number of items.
        max_items: usize,
    },
}

impl fmt::Display for CollectionBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyItems {
                max_items,
                actual_items,
            } => write!(
                f,
                "ordered collection exceeds the {max_items}-item limit: {actual_items} items"
            ),
            Self::AtCapacity { max_items } => {
                write!(f, "ordered collection is at its {max_items}-item limit")
            }
        }
    }
}

impl std::error::Error for CollectionBoundsError {}

/// Lossless error returned when a complete vector exceeds its item ceiling.
///
/// Debug output intentionally omits the rejected values.
pub struct BoundedVecBuildError<T> {
    values: Vec<T>,
    bounds: CollectionBoundsError,
}

impl<T> BoundedVecBuildError<T> {
    /// Return bounded metadata describing the rejection.
    #[must_use]
    pub const fn bounds(&self) -> CollectionBoundsError {
        self.bounds
    }

    /// Recover the rejected vector unchanged and in its original order.
    #[must_use]
    pub fn into_values(self) -> Vec<T> {
        self.values
    }
}

impl<T> fmt::Debug for BoundedVecBuildError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundedVecBuildError")
            .field("bounds", &self.bounds)
            .field("rejected_items", &self.values.len())
            .finish()
    }
}

impl<T> fmt::Display for BoundedVecBuildError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.bounds.fmt(f)
    }
}

impl<T> std::error::Error for BoundedVecBuildError<T> {}

/// Lossless error returned when an item is pushed at capacity.
///
/// Debug output intentionally omits the rejected value.
pub struct BoundedVecPushError<T> {
    value: T,
    bounds: CollectionBoundsError,
}

impl<T> BoundedVecPushError<T> {
    /// Return bounded metadata describing the rejection.
    #[must_use]
    pub const fn bounds(&self) -> CollectionBoundsError {
        self.bounds
    }

    /// Recover the rejected value unchanged.
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T> fmt::Debug for BoundedVecPushError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundedVecPushError")
            .field("bounds", &self.bounds)
            .finish()
    }
}

impl<T> fmt::Display for BoundedVecPushError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.bounds.fmt(f)
    }
}

impl<T> std::error::Error for BoundedVecPushError<T> {}

/// Owned ordered values whose item count cannot exceed `MAX_ITEMS`.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoundedVec<T, const MAX_ITEMS: usize>(Vec<T>);

impl<T, const MAX_ITEMS: usize> BoundedVec<T, MAX_ITEMS> {
    /// Construct an empty bounded ordered collection.
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Construct from a complete vector, returning it unchanged if over limit.
    pub fn try_from_vec(values: Vec<T>) -> Result<Self, BoundedVecBuildError<T>> {
        Self::try_from(values)
    }

    /// Append one value if capacity remains, otherwise return it unchanged.
    pub fn try_push(&mut self, value: T) -> Result<(), BoundedVecPushError<T>> {
        if self.0.len() >= MAX_ITEMS {
            return Err(BoundedVecPushError {
                value,
                bounds: CollectionBoundsError::AtCapacity {
                    max_items: MAX_ITEMS,
                },
            });
        }

        self.0.push(value);
        Ok(())
    }

    /// Borrow the ordered values as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// Return the current item count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Consume the wrapper and return the ordered values unchanged.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    /// Return the configured item-count ceiling.
    #[must_use]
    pub const fn max_items() -> usize {
        MAX_ITEMS
    }
}

impl<T, const MAX_ITEMS: usize> Default for BoundedVec<T, MAX_ITEMS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const MAX_ITEMS: usize> TryFrom<Vec<T>> for BoundedVec<T, MAX_ITEMS> {
    type Error = BoundedVecBuildError<T>;

    fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
        let actual_items = values.len();
        if actual_items > MAX_ITEMS {
            return Err(BoundedVecBuildError {
                values,
                bounds: CollectionBoundsError::TooManyItems {
                    max_items: MAX_ITEMS,
                    actual_items,
                },
            });
        }

        Ok(Self(values))
    }
}

impl<T, const MAX_ITEMS: usize> AsRef<[T]> for BoundedVec<T, MAX_ITEMS> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const MAX_ITEMS: usize> fmt::Debug for BoundedVec<T, MAX_ITEMS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundedVec")
            .field("max_items", &MAX_ITEMS)
            .field("len_items", &self.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash<T: Hash, const MAX_ITEMS: usize>(value: &BoundedVec<T, MAX_ITEMS>) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn construction_boundaries_are_lossless() {
        assert!(BoundedVec::<u8, 2>::try_from(vec![]).unwrap().is_empty());
        assert_eq!(
            BoundedVec::<u8, 2>::try_from(vec![1]).unwrap().as_slice(),
            &[1]
        );
        assert_eq!(
            BoundedVec::<u8, 2>::try_from(vec![1, 2])
                .unwrap()
                .as_slice(),
            &[1, 2]
        );

        let error = BoundedVec::<u8, 2>::try_from(vec![1, 2, 3]).unwrap_err();
        assert_eq!(
            error.bounds(),
            CollectionBoundsError::TooManyItems {
                max_items: 2,
                actual_items: 3,
            }
        );
        assert_eq!(error.into_values(), vec![1, 2, 3]);
    }

    #[test]
    fn zero_limit_and_failed_push_preserve_rejected_data() {
        assert!(
            BoundedVec::<String, 0>::try_from(Vec::new())
                .unwrap()
                .is_empty()
        );

        let rejected_values = vec![String::from("keep-me")];
        let build_error = BoundedVec::<String, 0>::try_from(rejected_values).unwrap_err();
        assert_eq!(
            build_error.bounds(),
            CollectionBoundsError::TooManyItems {
                max_items: 0,
                actual_items: 1,
            }
        );
        assert_eq!(build_error.into_values(), vec![String::from("keep-me")]);

        let mut collection = BoundedVec::<String, 0>::new();
        let push_error = collection.try_push(String::from("keep-me")).unwrap_err();
        assert!(collection.is_empty());
        assert_eq!(
            push_error.bounds(),
            CollectionBoundsError::AtCapacity { max_items: 0 }
        );
        assert_eq!(push_error.into_value(), "keep-me");
    }

    #[test]
    fn push_at_capacity_does_not_mutate_existing_values() {
        let mut collection =
            BoundedVec::<String, 2>::try_from(vec![String::from("first")]).unwrap();
        collection.try_push(String::from("second")).unwrap();

        let push_error = collection.try_push(String::from("third")).unwrap_err();
        assert_eq!(
            collection.as_slice(),
            &[String::from("first"), String::from("second")]
        );
        assert_eq!(
            push_error.bounds(),
            CollectionBoundsError::AtCapacity { max_items: 2 }
        );
        assert_eq!(push_error.into_value(), "third");
    }

    #[test]
    fn order_and_value_semantics_are_preserved() {
        let first = BoundedVec::<u8, 4>::try_from(vec![3, 1, 2]).unwrap();
        let same = BoundedVec::<u8, 4>::try_from(vec![3, 1, 2]).unwrap();
        let later = BoundedVec::<u8, 4>::try_from(vec![3, 2]).unwrap();

        assert_eq!(first.as_slice(), &[3, 1, 2]);
        assert_eq!(first.as_ref(), &[3, 1, 2]);
        assert_eq!(first, same);
        assert!(first < later);
        assert_eq!(hash(&first), hash(&same));
        assert_eq!(BoundedVec::<u8, 4>::max_items(), 4);
        assert_eq!(first.clone().into_vec(), vec![3, 1, 2]);
    }

    #[test]
    fn debug_and_error_diagnostics_do_not_disclose_values() {
        let secret = String::from("classified-item");
        let collection = BoundedVec::<String, 2>::try_from(vec![secret.clone()]).unwrap();
        let collection_debug = format!("{collection:?}");
        assert!(!collection_debug.contains(&secret));
        assert!(collection_debug.contains("max_items"));
        assert!(collection_debug.contains("len_items"));

        let build_error =
            BoundedVec::<String, 0>::try_from(vec![String::from("build-secret")]).unwrap_err();
        let build_debug = format!("{build_error:?}");
        let build_display = build_error.to_string();
        assert!(!build_debug.contains("build-secret"));
        assert!(!build_display.contains("build-secret"));

        let mut full =
            BoundedVec::<String, 1>::try_from(vec![String::from("existing-secret")]).unwrap();
        let push_error = full.try_push(String::from("rejected-secret")).unwrap_err();
        let push_debug = format!("{push_error:?}");
        let push_display = push_error.to_string();
        assert!(!push_debug.contains("rejected-secret"));
        assert!(!push_debug.contains("existing-secret"));
        assert!(!push_display.contains("rejected-secret"));
    }
}
