use criterion::{criterion_group, criterion_main, Criterion};
use json_array::Value;

pub fn bench(c: &mut Criterion) {
    let json = r#"[{"a":"foo"},{"b":"bar"},{"c":"baz"}]"#;
    c.bench_function("this/parse", |b| b.iter(|| json.parse::<Value>().unwrap()));
    c.bench_function("serde_json/parse", |b| {
        b.iter(|| json.parse::<serde_json::Value>().unwrap())
    });
    c.bench_function("jsonb/parse", |b| {
        b.iter(|| jsonb::parse_value(json.as_bytes()).unwrap().to_vec())
    });

    let v: Value = json.parse().unwrap();
    c.bench_function("this/[]->", |b| {
        b.iter(|| v.as_array().unwrap().get(2).unwrap().to_owned())
    });
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("serde_json/[]->", |b| {
        b.iter(|| v.as_array().unwrap().get(2).unwrap().to_owned())
    });
    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    c.bench_function("jsonb/[]->", |b| {
        b.iter(|| jsonb::get_by_index(&v, 2).unwrap())
    });

    let json = r#"{"a": {"b":"foo"}}"#;
    let v: Value = json.parse().unwrap();
    c.bench_function("this/{}->", |b| {
        b.iter(|| v.as_object().unwrap().get("a").unwrap().to_owned())
    });
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("serde_json/{}->", |b| {
        b.iter(|| v.as_object().unwrap().get("a").unwrap().to_owned())
    });
    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    c.bench_function("jsonb/{}->", |b| {
        b.iter(|| jsonb::get_by_name(&v, "a", false).unwrap())
    });

    let s = "1234567890";
    c.bench_function("this/from_string", |b| b.iter(|| Value::from(s)));
    c.bench_function("serde_json/from_string", |b| {
        b.iter(|| serde_json::Value::from(s))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
