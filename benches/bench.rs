use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use flat_json::ValueRef;

fn bench_parse(c: &mut Criterion) {
    for (filename, json) in iter_json_files() {
        c.bench_function(&format!("{filename} parse/this"), |b| {
            b.iter(|| json.parse::<flat_json::Value>().unwrap())
        });
        c.bench_function(&format!("{filename} parse/serde_json"), |b| {
            b.iter(|| json.parse::<serde_json::Value>().unwrap())
        });
        c.bench_function(&format!("{filename} parse/jsonb"), |b| {
            b.iter(|| jsonb::parse_value(json.as_bytes()).unwrap().to_vec())
        });
        c.bench_function(&format!("{filename} parse/simd-json"), |b| {
            b.iter_batched(
                || Vec::from(json.clone()),
                |mut data| simd_json::to_owned_value(&mut data).unwrap(),
                BatchSize::SmallInput,
            )
        });

        println!("{filename} size/text:\t{}", json.len());
        println!(
            "{filename} size/this:\t{}",
            json.parse::<flat_json::Value>().unwrap().capacity()
        );
        println!(
            "{filename} size/jsonb:\t{}",
            jsonb::parse_value(json.as_bytes()).unwrap().to_vec().len()
        );
    }
}

fn bench_to_string(c: &mut Criterion) {
    for (filename, json) in iter_json_files() {
        let v: flat_json::Value = json.parse().unwrap();
        c.bench_function(&format!("{filename} to_string/this"), |b| {
            b.iter(|| v.to_string())
        });
        let v: serde_json::Value = json.parse().unwrap();
        c.bench_function(&format!("{filename} to_string/serde_json"), |b| {
            b.iter(|| v.to_string())
        });
        let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
        c.bench_function(&format!("{filename} to_string/jsonb"), |b| {
            b.iter(|| jsonb::to_string(&v))
        });
    }
}

fn bench_from(c: &mut Criterion) {
    let s = "1234567890";
    c.bench_function("from_string/this", |b| b.iter(|| flat_json::Value::from(s)));
    c.bench_function("from_string/serde_json", |b| {
        b.iter(|| serde_json::Value::from(s))
    });
    c.bench_function("from_string/jsonb", |b| {
        b.iter(|| jsonb::Value::from(s).to_vec())
    });

    let s = 123456789012345678_i64;
    c.bench_function("from_i64/this", |b| b.iter(|| flat_json::Value::from(s)));
    c.bench_function("from_i64/serde_json", |b| {
        b.iter(|| serde_json::Value::from(s))
    });
    c.bench_function("from_i64/jsonb", |b| {
        b.iter(|| jsonb::Value::from(s).to_vec())
    });

    let s = 1234567890.1234567890;
    c.bench_function("from_f64/this", |b| b.iter(|| flat_json::Value::from(s)));
    c.bench_function("from_f64/serde_json", |b| {
        b.iter(|| serde_json::Value::from(s))
    });
    c.bench_function("from_f64/jsonb", |b| {
        b.iter(|| jsonb::Value::from(s).to_vec())
    });
}

fn bench_index(c: &mut Criterion) {
    let json = r#"[{"a":"foo"},{"b":"bar"},{"c":"baz"}]"#;
    let v: flat_json::Value = json.parse().unwrap();
    c.bench_function("json[0]/this", |b| b.iter(|| v.get(2).unwrap().to_owned()));
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("json[0]/serde_json", |b| {
        b.iter(|| v.get(2).unwrap().to_owned())
    });
    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    c.bench_function("json[0]/jsonb", |b| {
        b.iter(|| jsonb::get_by_index(&v, 2).unwrap())
    });

    let json = r#"{"a": {"b":"foo"}}"#;
    let v: flat_json::Value = json.parse().unwrap();
    c.bench_function("json['key']/this", |b| {
        b.iter(|| v.get("a").unwrap().to_owned())
    });
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("json['key']/serde_json", |b| {
        b.iter(|| v.get("a").unwrap().to_owned())
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

    let v: flat_json::Value = json.parse().unwrap();
    let mut array = vec![];
    let mut index = vec![];
    for _ in 0..n {
        let start = array.len();
        array.extend_from_slice(v.as_slice());
        let end = array.len();
        index.push(start..end);
    }
    c.bench_function("[json['key'] for json in array]/this", |b| {
        b.iter(|| {
            let mut buffer = Vec::with_capacity(array.len());
            for range in index.iter() {
                let value = unsafe { ValueRef::from_slice(&array[range.clone()]) };
                let mut builder = flat_json::Builder::new(&mut buffer);
                let new_value = value.get("name").unwrap();
                builder.add_value_ref(new_value);
                builder.finish();
            }
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
            let mut new_array = Vec::with_capacity(array.len());
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
            v.get("a")
                .unwrap()
                .get("b")
                .unwrap()
                .get(1)
                .unwrap()
                .to_owned()
        })
    });
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("json[path]/serde_json", |b| {
        b.iter(|| v["a"]["b"][1].to_owned())
    });
    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    c.bench_function("json[path]/jsonb", |b| {
        let path = jsonb::jsonpath::parse_json_path("{a,b,1}".as_bytes()).unwrap();
        b.iter(|| jsonb::get_by_path(&v, path.clone(), &mut vec![], &mut vec![]))
    });
}

/// Index JSONs loaded from file.
fn bench_file_index(c: &mut Criterion) {
    struct TestSuite {
        file: &'static str,
        paths: &'static [&'static str],
        expected: Option<&'static str>,
    }
    let test_suites = &[
        TestSuite {
            file: "canada",
            paths: &["type"],
            expected: Some("FeatureCollection"),
        },
        TestSuite {
            file: "citm_catalog",
            paths: &["areaNames"],
            expected: None,
        },
        TestSuite {
            file: "citm_catalog",
            paths: &["areaNames", "205705994"],
            expected: Some("1er balcon central"),
        },
        TestSuite {
            file: "citm_catalog",
            paths: &["topicNames", "324846100"],
            expected: Some("Formations musicales"),
        },
        TestSuite {
            file: "twitter",
            paths: &["search_metadata", "max_id_str"],
            expected: Some("505874924095815681"),
        },
    ];

    for test_suite in test_suites {
        let suite_name = format!("{}->{}", test_suite.file, test_suite.paths.join("->"));
        let bytes = std::fs::read(&format!("./benches/data/{}.json", test_suite.file)).unwrap();

        let value: flat_json::Value = std::str::from_utf8(&bytes).unwrap().parse().unwrap();
        c.bench_function(&format!("{suite_name} index/this"), |b| {
            let bench = || {
                let mut v = value.as_ref();
                for path in test_suite.paths {
                    v = v.get(path).unwrap();
                }
                v.to_owned()
            };
            if let Some(expected) = test_suite.expected {
                assert_eq!(bench().as_str(), Some(expected));
            }
            b.iter(bench);
        });

        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        c.bench_function(&format!("{suite_name} index/serde_json"), |b| {
            let bench = || {
                let mut v = &value;
                for path in test_suite.paths {
                    v = v.get(path).unwrap();
                }
                v.to_owned()
            };
            if let Some(expected) = test_suite.expected {
                assert_eq!(bench().as_str(), Some(expected));
            }
            b.iter(bench);
        });

        let jsonb = jsonb::parse_value(&bytes).unwrap().to_vec();
        let json_path = jsonb::jsonpath::JsonPath {
            paths: test_suite
                .paths
                .iter()
                .map(|p| jsonb::jsonpath::Path::DotField(std::borrow::Cow::Borrowed(p)))
                .collect(),
        };
        c.bench_function(&format!("{suite_name} index/jsonb"), |b| {
            let bench = || {
                let mut data = vec![];
                jsonb::get_by_path(&jsonb, json_path.clone(), &mut data, &mut vec![]);
                data
            };
            if let Some(expected) = test_suite.expected {
                assert_eq!(jsonb::as_str(&bench()), Some(expected.into()));
            }
            b.iter(bench);
        });
    }
}

/// Iterate over all files in the `./benches/data/` directory.
fn iter_json_files() -> impl Iterator<Item = (String, String)> {
    std::fs::read_dir("./benches/data/").unwrap().map(|path| {
        let path = path.unwrap().path();
        let filename = path.file_stem().unwrap().to_str().unwrap();
        let json = std::fs::read_to_string(&path).unwrap();
        (filename.to_owned(), json)
    })
}

criterion_group!(
    benches,
    bench_from,
    bench_parse,
    bench_to_string,
    bench_index,
    bench_index_array,
    bench_file_index,
    bench_path
);
criterion_main!(benches);
