#![no_main]

use libfuzzer_sys::fuzz_target;
use morize_core::{BoundedVec, CollectionBoundsError};

fn check<const MAX_ITEMS: usize>(data: &[u8]) {
    let original = data.to_vec();

    match BoundedVec::<u8, MAX_ITEMS>::try_from(original.clone()) {
        Ok(value) => {
            assert!(original.len() <= MAX_ITEMS);
            assert_eq!(value.len(), original.len());
            assert_eq!(value.as_slice(), original.as_slice());
            assert_eq!(value.into_vec(), original);
        }
        Err(error) => {
            assert!(original.len() > MAX_ITEMS);
            match error.bounds() {
                CollectionBoundsError::TooManyItems {
                    max_items,
                    actual_items,
                } => {
                    assert_eq!(max_items, MAX_ITEMS);
                    assert_eq!(actual_items, original.len());
                }
                _ => panic!("unexpected BoundedVec build error variant"),
            }
            assert_eq!(error.into_values(), original);
        }
    }

    let mut incremental = BoundedVec::<u8, MAX_ITEMS>::new();
    for (index, value) in data.iter().copied().enumerate() {
        let before = incremental.as_slice().to_vec();
        match incremental.try_push(value) {
            Ok(()) => {
                assert!(index < MAX_ITEMS);
                assert_eq!(incremental.len(), index + 1);
                assert_eq!(incremental.as_slice(), &data[..=index]);
            }
            Err(error) => {
                assert!(index >= MAX_ITEMS);
                assert_eq!(incremental.as_slice(), before.as_slice());
                match error.bounds() {
                    CollectionBoundsError::AtCapacity { max_items } => {
                        assert_eq!(max_items, MAX_ITEMS);
                    }
                    _ => panic!("unexpected BoundedVec push error variant"),
                }
                assert_eq!(error.into_value(), value);
            }
        }
    }
}

fuzz_target!(|data: &[u8]| {
    check::<0>(data);
    check::<1>(data);
    check::<2>(data);
    check::<4>(data);
    check::<8>(data);
    check::<16>(data);
});
