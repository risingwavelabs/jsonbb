use criterion::{criterion_group, criterion_main, Criterion};
use json_array::Value;

pub fn bench(c: &mut Criterion) {
    let json = r#"[{"a":"foo"},{"b":"bar"},{"c":"baz"}]"#;
    let v: Value = json.parse().unwrap();
    c.bench_function("this/parse", |b| b.iter(|| json.parse::<Value>().unwrap()));
    c.bench_function("this/->", |b| {
        b.iter(|| v.as_array().unwrap().get(2).unwrap().to_owned())
    });

    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("serde_json/parse", |b| {
        b.iter(|| json.parse::<serde_json::Value>().unwrap())
    });
    c.bench_function("serde_json/->", |b| {
        b.iter(|| v.as_array().unwrap().get(2).unwrap().to_owned())
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
