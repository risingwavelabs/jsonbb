use criterion::{criterion_group, criterion_main, Criterion};

fn bench_parse(c: &mut Criterion) {
    let json = r#"[{"a":"foo"},{"b":"bar"},{"c":"baz"}]"#;
    c.bench_function("parse/this", |b| {
        b.iter(|| json.parse::<flat_json::Value>().unwrap())
    });
    c.bench_function("parse/serde_json", |b| {
        b.iter(|| json.parse::<serde_json::Value>().unwrap())
    });
    c.bench_function("parse/jsonb", |b| {
        b.iter(|| jsonb::parse_value(json.as_bytes()).unwrap().to_vec())
    });

    println!(
        "capacity/this: {}",
        json.parse::<flat_json::Value>().unwrap().capacity()
    );
    println!(
        "capacity/jsonb: {}",
        jsonb::parse_value(json.as_bytes()).unwrap().to_vec().len()
    );
}

fn bench_from(c: &mut Criterion) {
    let s = "1234567890";
    c.bench_function("from_string/this", |b| b.iter(|| flat_json::Value::from(s)));
    c.bench_function("from_string/serde_json", |b| {
        b.iter(|| serde_json::Value::from(s))
    });
}

fn bench_index(c: &mut Criterion) {
    let json = r#"[{"a":"foo"},{"b":"bar"},{"c":"baz"}]"#;
    let v: flat_json::Value = json.parse().unwrap();
    c.bench_function("json[0]/this", |b| {
        b.iter(|| v.as_array().unwrap().get(2).unwrap().to_owned())
    });
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("json[0]/serde_json", |b| {
        b.iter(|| v.as_array().unwrap().get(2).unwrap().to_owned())
    });
    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    c.bench_function("json[0]/jsonb", |b| {
        b.iter(|| jsonb::get_by_index(&v, 2).unwrap())
    });

    let json = r#"{"a": {"b":"foo"}}"#;
    let v: flat_json::Value = json.parse().unwrap();
    c.bench_function("json['key']/this", |b| {
        b.iter(|| v.as_object().unwrap().get("a").unwrap().to_owned())
    });
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("json['key']/serde_json", |b| {
        b.iter(|| v.as_object().unwrap().get("a").unwrap().to_owned())
    });
    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    c.bench_function("json['key']/jsonb", |b| {
        b.iter(|| jsonb::get_by_name(&v, "a", false).unwrap())
    });
}

fn bench_index_array(c: &mut Criterion) {
    let json = r#"
    {
        "age": 43,
        "name": "John Doe",
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;
    let n = 1024;

    let array = {
        let v: flat_json::Value = json.parse().unwrap();
        let mut builder = flat_json::Builder::default();
        let ids = (0..n)
            .map(|_| builder.add_value_ref(v.as_ref()))
            .collect::<Vec<_>>();
        builder.finish_array(&ids)
    };
    c.bench_function("[json['key'] for json in array]/this", |b| {
        b.iter(|| {
            let mut builder = flat_json::Builder::default();
            let mut ids = Vec::with_capacity(array.len());
            for value in array.iter() {
                let new_value = value.as_object().unwrap().get("name").unwrap();
                let id = builder.add_value_ref(new_value);
                ids.push(id);
            }
            builder.finish_array(&ids)
        })
    });

    let v: serde_json::Value = json.parse().unwrap();
    let array = vec![v; n];
    c.bench_function("[json['key'] for json in array]/serde_json", |b| {
        b.iter(|| {
            array
                .iter()
                .map(|v| v["name"].to_owned())
                .collect::<Vec<serde_json::Value>>()
        })
    });

    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    let mut array = vec![];
    let mut index = vec![];
    for _ in 0..n {
        let start = array.len();
        array.extend_from_slice(&v);
        let end = array.len();
        index.push(start..end);
    }
    c.bench_function("[json['key'] for json in array]/jsonb", |b| {
        b.iter(|| {
            let mut new_array = vec![];
            for range in index.iter() {
                let new_value = jsonb::get_by_name(&array[range.clone()], "name", false).unwrap();
                new_array.extend_from_slice(&new_value);
            }
            new_array
        })
    });
}

fn bench_path(c: &mut Criterion) {
    let json = r#"{"a": {"b": ["foo","bar"]}}"#;
    let v: flat_json::Value = json.parse().unwrap();
    c.bench_function("json[path]/this", |b| {
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
    c.bench_function("json[path]/serde_json", |b| {
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
    c.bench_function("json[path]/jsonb", |b| {
        b.iter(|| {
            // TODO: parsing is slow
            let path = jsonb::jsonpath::parse_json_path("{a,b,1}".as_bytes()).unwrap();
            jsonb::get_by_path(&v, path)
        })
    });
}

criterion_group!(
    benches,
    bench_from,
    bench_parse,
    bench_index,
    bench_index_array,
    bench_path
);
criterion_main!(benches);
