#![no_main]

use libfuzzer_sys::fuzz_target;
use morize_core::{BoundedUtf8, Utf8BoundsError};

fn check<const MAX_BYTES: usize>(text: &str) {
    let actual_bytes = text.len();
    match BoundedUtf8::<MAX_BYTES>::try_from(text) {
        Ok(value) => {
            assert!(actual_bytes <= MAX_BYTES);
            assert_eq!(value.len_bytes(), actual_bytes);
            assert_eq!(value.as_str().as_bytes(), text.as_bytes());
            assert_eq!(value.into_string().as_bytes(), text.as_bytes());
        }
        Err(error) => {
            assert!(actual_bytes > MAX_BYTES);
            match error {
                Utf8BoundsError::TooLong {
                    max_bytes,
                    actual_bytes: rejected_bytes,
                } => {
                    assert_eq!(max_bytes, MAX_BYTES);
                    assert_eq!(rejected_bytes, actual_bytes);
                }
                _ => panic!("unexpected BoundedUtf8 error variant"),
            }
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let Ok(text) = core::str::from_utf8(data) else {
        return;
    };

    check::<0>(text);
    check::<1>(text);
    check::<2>(text);
    check::<3>(text);
    check::<4>(text);
    check::<5>(text);
    check::<8>(text);
    check::<16>(text);
    check::<32>(text);
    check::<64>(text);
});
