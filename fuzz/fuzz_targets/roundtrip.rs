#![no_main]

use jsonbb::Value;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Restrict size to keep allocations bounded during fuzzing.
    if data.len() > 1 << 16 {
        return;
    }

    let input = match std::str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return,
    };

    if !input.starts_with('{') {
        return;
    }

    let value = match Value::from_text(input.as_bytes()) {
        Ok(v) => v,
        Err(_) => return,
    };

    let original = value.as_ref();
    let text = original.to_string();

    let roundtripped = match Value::from_text(text.as_bytes()) {
        Ok(v) => v,
        Err(_) => return,
    };

    let roundtripped = roundtripped.as_ref();

    assert_eq!(
        original, roundtripped,
        "input: {input}\noriginal: {original}\nroundtripped: {roundtripped}"
    );
});
