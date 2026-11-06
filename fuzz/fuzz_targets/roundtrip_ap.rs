#![no_main]

use jsonbb::Value;
use libfuzzer_sys::fuzz_target;
use std::str::FromStr as _;

fuzz_target!(|data: &[u8]| {
    let Ok(original) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(_) = f64::from_str(original) else {
        return;
    };
    let original = original
        .replace(|c: char| c.is_whitespace(), "")
        .replace("-0", "0");

    let Ok(value) = Value::from_str(&original) else {
        return;
    };
    let roundtripped = value.to_string();

    assert_eq!(
        original, roundtripped,
        "original: {original}\nroundtripped: {roundtripped}"
    );
});
