#![no_main]

use arbitrary_json::ArbitraryValue;
use jsonbb::Value;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: ArbitraryValue| {
    let input = data.to_string();

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
