// Copyright 2023 RisingWave Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use jsonbb::ValueRef;
use simd_json::{Mutable, ValueAccess};

fn bench_parse(c: &mut Criterion) {
    for (filename, json) in iter_json_files() {
        c.bench_function(&format!("{filename} parse/jsonbb"), |b| {
            b.iter(|| json.parse::<jsonbb::Value>().unwrap())
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
            "{filename} size/jsonbb:\t{}",
            json.parse::<jsonbb::Value>().unwrap().capacity()
        );
        println!(
            "{filename} size/jsonb:\t{}",
            jsonb::parse_value(json.as_bytes()).unwrap().to_vec().len()
        );
    }
}

fn bench_to_string(c: &mut Criterion) {
    for (filename, json) in iter_json_files() {
        let v: jsonbb::Value = json.parse().unwrap();
        c.bench_function(&format!("{filename} to_string/jsonbb"), |b| {
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
        let v = simd_json::to_owned_value(&mut Vec::from(json.clone())).unwrap();
        c.bench_function(&format!("{filename} to_string/simd-json"), |b| {
            b.iter(|| simd_json::to_string(&v).unwrap())
        });
    }
}

fn bench_hash(c: &mut Criterion) {
    use std::hash::{Hash, Hasher};

    let json = r#"[{"a":"foo"},{"b":"bar"},{"c":"baz"}]"#;

    fn hash(v: &impl Hash) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        v.hash(&mut hasher);
        hasher.finish()
    }

    let v: jsonbb::Value = json.parse().unwrap();
    c.bench_function("hash/jsonbb", |b| b.iter(|| hash(&v)));

    // other crates don't implement Hash
}

fn bench_eq(c: &mut Criterion) {
    let json1 = r#"{"a":"foo","b":[null,1,"bar"]}"#;
    let json2 = r#"{"b":[null,1,"bar"],"a":"foo"}"#;

    let v1: jsonbb::Value = json1.parse().unwrap();
    let v2: jsonbb::Value = json2.parse().unwrap();
    assert_eq!(v1, v2);
    c.bench_function("eq/jsonbb", |b| b.iter(|| v1 == v2));

    let v1: serde_json::Value = json1.parse().unwrap();
    let v2: serde_json::Value = json2.parse().unwrap();
    assert_eq!(v1, v2);
    c.bench_function("eq/serde_json", |b| b.iter(|| v1 == v2));

    let v1 = jsonb::parse_value(json1.as_bytes()).unwrap().to_vec();
    let v2 = jsonb::parse_value(json2.as_bytes()).unwrap().to_vec();
    assert_eq!(v1, v2);
    c.bench_function("eq/jsonb", |b| b.iter(|| jsonb::compare(&v1, &v2)));

    let v1 = simd_json::to_owned_value(&mut Vec::from(json1)).unwrap();
    let v2 = simd_json::to_owned_value(&mut Vec::from(json2)).unwrap();
    assert_eq!(v1, v2);
    c.bench_function("eq/simd-json", |b| b.iter(|| v1 == v2));
}

fn bench_cmp(c: &mut Criterion) {
    let json1 = r#"{"a":"foo","b":[null,1,"bar"]}"#;
    let json2 = r#"{"a":"foo","b":[null,1,"baz"]}"#;

    let v1: jsonbb::Value = json1.parse().unwrap();
    let v2: jsonbb::Value = json2.parse().unwrap();
    assert!(v1 < v2);
    c.bench_function("cmp/jsonbb", |b| b.iter(|| v1 < v2));

    let v1 = jsonb::parse_value(json1.as_bytes()).unwrap().to_vec();
    let v2 = jsonb::parse_value(json2.as_bytes()).unwrap().to_vec();
    assert!(jsonb::compare(&v1, &v2).unwrap().is_lt());
    c.bench_function("cmp/jsonb", |b| b.iter(|| jsonb::compare(&v1, &v2)));

    // serde_json and simd_json don't implement Ord
}

fn bench_from(c: &mut Criterion) {
    let s = "1234567890";
    c.bench_function("from_string/jsonbb", |b| b.iter(|| jsonbb::Value::from(s)));
    c.bench_function("from_string/serde_json", |b| {
        b.iter(|| serde_json::Value::from(s))
    });
    c.bench_function("from_string/jsonb", |b| {
        b.iter(|| jsonb::Value::from(s).to_vec())
    });
    c.bench_function("from_string/simd-json", |b| {
        b.iter(|| simd_json::OwnedValue::from(s))
    });

    let s = 123456789012345678_i64;
    c.bench_function("from_i64/jsonbb", |b| b.iter(|| jsonbb::Value::from(s)));
    c.bench_function("from_i64/serde_json", |b| {
        b.iter(|| serde_json::Value::from(s))
    });
    c.bench_function("from_i64/jsonb", |b| {
        b.iter(|| jsonb::Value::from(s).to_vec())
    });
    c.bench_function("from_i64/simd-json", |b| {
        b.iter(|| simd_json::OwnedValue::from(s))
    });

    let s = 1_234_567_890.123_456_7;
    c.bench_function("from_f64/jsonbb", |b| b.iter(|| jsonbb::Value::from(s)));
    c.bench_function("from_f64/serde_json", |b| {
        b.iter(|| serde_json::Value::from(s))
    });
    c.bench_function("from_f64/jsonb", |b| {
        b.iter(|| jsonb::Value::from(s).to_vec())
    });
    c.bench_function("from_f64/simd-json", |b| {
        b.iter(|| simd_json::OwnedValue::from(s))
    });
}

fn bench_index(c: &mut Criterion) {
    let json = r#"[{"a":"foo"},{"b":"bar"},{"c":"baz"}]"#;
    let v: jsonbb::Value = json.parse().unwrap();
    c.bench_function("json[i]/jsonbb", |b| {
        b.iter(|| v.get(2).unwrap().to_owned())
    });
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("json[i]/serde_json", |b| {
        b.iter(|| v.get(2).unwrap().to_owned())
    });
    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    c.bench_function("json[i]/jsonb", |b| {
        b.iter(|| jsonb::get_by_index(&v, 2).unwrap())
    });
    let v = simd_json::to_owned_value(&mut Vec::from(json)).unwrap();
    c.bench_function("json[i]/simd-json", |b| {
        b.iter(|| v.get_idx(2).unwrap().to_owned())
    });

    let json = r#"{"a": 1, "b": 2, "c": 3, "d": 4, "e": 5, "f": {"a":"foo"}}"#;
    let v: jsonbb::Value = json.parse().unwrap();
    c.bench_function("json['key']/jsonbb", |b| {
        b.iter(|| v.get("f").unwrap().to_owned())
    });
    let v: serde_json::Value = json.parse().unwrap();
    c.bench_function("json['key']/serde_json", |b| {
        b.iter(|| v.get("f").unwrap().to_owned())
    });
    let v = jsonb::parse_value(json.as_bytes()).unwrap().to_vec();
    c.bench_function("json['key']/jsonb", |b| {
        b.iter(|| jsonb::get_by_name(&v, "f", false).unwrap())
    });
    let v = simd_json::to_owned_value(&mut Vec::from(json)).unwrap();
    c.bench_function("json['key']/simd-json", |b| {
        b.iter(|| v.get("f").unwrap().to_owned())
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

    let v: jsonbb::Value = json.parse().unwrap();
    let mut array = vec![];
    let mut index = vec![];
    for _ in 0..n {
        let start = array.len();
        array.extend_from_slice(v.as_bytes());
        let end = array.len();
        index.push(start..end);
    }
    c.bench_function("[json['key'] for json in array]/jsonbb", |b| {
        b.iter(|| {
            let mut buffer = Vec::with_capacity(array.len());
            for range in index.iter() {
                let value = ValueRef::from_bytes(&array[range.clone()]);
                let mut builder = jsonbb::Builder::<&mut Vec<u8>>::new(&mut buffer);
                let new_value = value.get("name").unwrap();
                builder.add_value(new_value);
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
    let v: jsonbb::Value = json.parse().unwrap();
    c.bench_function("json[path]/jsonbb", |b| {
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
    let v = simd_json::to_owned_value(&mut Vec::from(json)).unwrap();
    c.bench_function("json[path]/simd-json", |b| {
        b.iter(|| v["a"]["b"][1].to_owned())
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

        let value: jsonbb::Value = std::str::from_utf8(&bytes).unwrap().parse().unwrap();
        c.bench_function(&format!("{suite_name} index/jsonbb"), |b| {
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

        let value = simd_json::to_owned_value(&mut bytes.clone()).unwrap();
        c.bench_function(&format!("{suite_name} index/simd-json"), |b| {
            let bench = || {
                let mut v = &value;
                for path in test_suite.paths {
                    v = v.get(*path).unwrap();
                }
                v.to_owned()
            };
            if let Some(expected) = test_suite.expected {
                match bench() {
                    simd_json::OwnedValue::String(s) => assert_eq!(s, expected),
                    _ => panic!("expected string"),
                }
            }
            b.iter(bench);
        });
    }
}

fn bench_array_push(c: &mut Criterion) {
    let array = r#"[{"a":"foo"},{"b":"bar"},{"c":"baz"}]"#;
    let value = r#"{"d":"qqq"}"#;

    let a: jsonbb::Value = array.parse().unwrap();
    let v: jsonbb::Value = value.parse().unwrap();
    c.bench_function("array_push/jsonbb", |b| {
        b.iter_batched(
            || a.clone(),
            |mut a| a.array_push(v.as_ref()),
            BatchSize::SmallInput,
        )
    });

    let a: serde_json::Value = array.parse().unwrap();
    let v: serde_json::Value = value.parse().unwrap();
    c.bench_function("array_push/serde_json", |b| {
        b.iter_batched(
            || a.clone(),
            |mut a| a.as_array_mut().unwrap().push(v.clone()),
            BatchSize::SmallInput,
        )
    });

    let a = jsonb::parse_value(array.as_bytes()).unwrap().to_vec();
    let v = jsonb::parse_value(value.as_bytes()).unwrap().to_vec();
    c.bench_function("array_push/jsonb", |b| {
        b.iter(|| {
            let elems = jsonb::array_values(&a).unwrap();
            let mut buf = Vec::with_capacity(a.len() + v.len());
            jsonb::build_array(
                elems.iter().map(|v| v.as_slice()).chain([v.as_slice()]),
                &mut buf,
            )
            .unwrap();
            buf
        })
    });

    let a = simd_json::to_owned_value(&mut Vec::from(array)).unwrap();
    let v = simd_json::to_owned_value(&mut Vec::from(value)).unwrap();
    c.bench_function("array_push/simd-json", |b| {
        b.iter_batched(
            || a.clone(),
            |mut a| a.as_array_mut().unwrap().push(v.clone()),
            BatchSize::SmallInput,
        )
    });
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
    bench_hash,
    bench_eq,
    bench_cmp,
    bench_index,
    bench_index_array,
    bench_file_index,
    bench_path,
    bench_array_push
);
criterion_main!(benches);
