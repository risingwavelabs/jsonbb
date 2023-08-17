use criterion::{criterion_group, criterion_main, Criterion};

fn bench_parse(c: &mut Criterion) {
    let json = r#"[{"a":"foo"},{"b":"bar"},{"c":"baz"}]"#;
    c.bench_function("parse/this", |b| {
        b.iter(|| json.parse::<json_array::Value>().unwrap())
    });
    c.bench_function("parse/serde_json", |b| {
        b.iter(|| json.parse::<serde_json::Value>().unwrap())
    });
    c.bench_function("parse/jsonb", |b| {
        b.iter(|| jsonb::parse_value(json.as_bytes()).unwrap().to_vec())
    });

    println!(
        "capacity/this: {}",
        json.parse::<json_array::Value>().unwrap().capacity()
    );
    println!(
        "capacity/jsonb: {}",
        jsonb::parse_value(json.as_bytes()).unwrap().to_vec().len()
    );
}

fn bench_from(c: &mut Criterion) {
    let s = "1234567890";
    c.bench_function("from_string/this", |b| {
        b.iter(|| json_array::Value::from(s))
    });
    c.bench_function("from_string/serde_json", |b| {
        b.iter(|| serde_json::Value::from(s))
    });
}

fn bench_index(c: &mut Criterion) {
    let json = r#"[{"a":"foo"},{"b":"bar"},{"c":"baz"}]"#;
    let v: json_array::Value = json.parse().unwrap();
    c.bench_function("[]->/this", |b| {
        b.iter(|| v.as_array().unwrap().get(2).unwrap().to_owned())
    });
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("[]->/serde_json", |b| {
        b.iter(|| v.as_array().unwrap().get(2).unwrap().to_owned())
    });
    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    c.bench_function("[]->/jsonb", |b| {
        b.iter(|| jsonb::get_by_index(&v, 2).unwrap())
    });

    let json = r#"{"a": {"b":"foo"}}"#;
    let v: json_array::Value = json.parse().unwrap();
    c.bench_function("{}->/this", |b| {
        b.iter(|| v.as_object().unwrap().get("a").unwrap().to_owned())
    });
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("{}->/serde_json", |b| {
        b.iter(|| v.as_object().unwrap().get("a").unwrap().to_owned())
    });
    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    c.bench_function("{}->/jsonb", |b| {
        b.iter(|| jsonb::get_by_name(&v, "a", false).unwrap())
    });
}

fn bench_path(c: &mut Criterion) {
    let json = r#"{"a": {"b": ["foo","bar"]}}"#;
    let v: json_array::Value = json.parse().unwrap();
    c.bench_function("#>/this", |b| {
        b.iter(|| {
            v.as_object()
                .unwrap()
                .get("a")
                .unwrap()
                .as_object()
                .unwrap()
                .get("b")
                .unwrap()
                .as_array()
                .unwrap()
                .get(1)
                .unwrap()
                .to_owned()
        })
    });
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("#>/serde_json", |b| {
        b.iter(|| {
            v.as_object()
                .unwrap()
                .get("a")
                .unwrap()
                .get("b")
                .unwrap()
                .get(1)
                .unwrap()
                .to_owned()
        })
    });
    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    c.bench_function("#>/jsonb", |b| {
        b.iter(|| {
            // TODO: parsing is slow
            let path = jsonb::jsonpath::parse_json_path("{a,b,1}".as_bytes()).unwrap();
            jsonb::get_by_path(&v, path)
        })
    });
}

criterion_group!(benches, bench_from, bench_parse, bench_index, bench_path);
criterion_main!(benches);
