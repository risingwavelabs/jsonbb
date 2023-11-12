window.BENCHMARK_DATA = {
  "lastUpdate": 1699792251385,
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
      }
    ]
  }
}