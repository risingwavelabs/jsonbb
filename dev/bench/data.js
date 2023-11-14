window.BENCHMARK_DATA = {
  "lastUpdate": 1699965838858,
  "repoUrl": "https://github.com/risingwavelabs/jsonbb",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "wangrunji0408@163.com",
            "name": "Runji Wang",
            "username": "wangrunji0408"
          },
          "committer": {
            "email": "wangrunji0408@163.com",
            "name": "Runji Wang",
            "username": "wangrunji0408"
          },
          "distinct": true,
          "id": "f25173f91612d89e280e508cf77f51029590bff9",
          "message": "add Github Actions for benchmark\n\nSigned-off-by: Runji Wang <wangrunji0408@163.com>",
          "timestamp": "2023-11-12T20:24:44+08:00",
          "tree_id": "7366a1621a988f4da01eb72b8d228f80e245380e",
          "url": "https://github.com/risingwavelabs/jsonbb/commit/f25173f91612d89e280e508cf77f51029590bff9"
        },
        "date": 1699792250702,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_string/jsonbb",
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "from_i64/jsonbb",
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "from_f64/jsonbb",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "canada parse/jsonbb",
            "value": 7384641,
            "range": "± 62175",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog parse/jsonbb",
            "value": 3790083,
            "range": "± 4253",
            "unit": "ns/iter"
          },
          {
            "name": "twitter parse/jsonbb",
            "value": 2664647,
            "range": "± 1754",
            "unit": "ns/iter"
          },
          {
            "name": "canada to_string/jsonbb",
            "value": 11714842,
            "range": "± 48710",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog to_string/jsonbb",
            "value": 3530403,
            "range": "± 4367",
            "unit": "ns/iter"
          },
          {
            "name": "twitter to_string/jsonbb",
            "value": 2021376,
            "range": "± 3723",
            "unit": "ns/iter"
          },
          {
            "name": "hash/jsonbb",
            "value": 224,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "eq/jsonbb",
            "value": 157,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cmp/jsonbb",
            "value": 170,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json[i]/jsonbb",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json['key']/jsonbb",
            "value": 74,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "[json['key'] for json in array]/jsonbb",
            "value": 42832,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "canada->type index/jsonbb",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->areaNames index/jsonbb",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->areaNames->205705994 index/jsonbb",
            "value": 143,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->topicNames->324846100 index/jsonbb",
            "value": 107,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "twitter->search_metadata->max_id_str index/jsonbb",
            "value": 109,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json[path]/jsonbb",
            "value": 103,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "array_push/jsonbb",
            "value": 156,
            "range": "± 2",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangrunji0408@163.com",
            "name": "Runji Wang",
            "username": "wangrunji0408"
          },
          "committer": {
            "email": "wangrunji0408@163.com",
            "name": "Runji Wang",
            "username": "wangrunji0408"
          },
          "distinct": true,
          "id": "f826d68fa58e839bfd01d718e2791ef99be48eb3",
          "message": "add `is_*` methods\n\nSigned-off-by: Runji Wang <wangrunji0408@163.com>",
          "timestamp": "2023-11-14T13:37:36+08:00",
          "tree_id": "fcc828aa87e30d04d448a8ba1d7dac053bece0d3",
          "url": "https://github.com/risingwavelabs/jsonbb/commit/f826d68fa58e839bfd01d718e2791ef99be48eb3"
        },
        "date": 1699965837861,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_string/jsonbb",
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "from_i64/jsonbb",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "from_f64/jsonbb",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "canada parse/jsonbb",
            "value": 5748343,
            "range": "± 185751",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog parse/jsonbb",
            "value": 2807104,
            "range": "± 32388",
            "unit": "ns/iter"
          },
          {
            "name": "twitter parse/jsonbb",
            "value": 1861917,
            "range": "± 52370",
            "unit": "ns/iter"
          },
          {
            "name": "canada to_string/jsonbb",
            "value": 10561397,
            "range": "± 36606",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog to_string/jsonbb",
            "value": 2707246,
            "range": "± 9711",
            "unit": "ns/iter"
          },
          {
            "name": "twitter to_string/jsonbb",
            "value": 1523083,
            "range": "± 52004",
            "unit": "ns/iter"
          },
          {
            "name": "hash/jsonbb",
            "value": 212,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "eq/jsonbb",
            "value": 157,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cmp/jsonbb",
            "value": 153,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "json[i]/jsonbb",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json['key']/jsonbb",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "[json['key'] for json in array]/jsonbb",
            "value": 39688,
            "range": "± 161",
            "unit": "ns/iter"
          },
          {
            "name": "canada->type index/jsonbb",
            "value": 68,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->areaNames index/jsonbb",
            "value": 95,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->areaNames->205705994 index/jsonbb",
            "value": 128,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->topicNames->324846100 index/jsonbb",
            "value": 111,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "twitter->search_metadata->max_id_str index/jsonbb",
            "value": 117,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json[path]/jsonbb",
            "value": 110,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "array_push/jsonbb",
            "value": 113,
            "range": "± 2",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}