window.BENCHMARK_DATA = {
  "lastUpdate": 1703087552831,
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
          "id": "e7c18e8b25f1a1072cca732ae6e030f00bf2fd62",
          "message": "revert the wrong change for `as_u64`...\n\nSigned-off-by: Runji Wang <wangrunji0408@163.com>",
          "timestamp": "2023-11-15T17:00:45+08:00",
          "tree_id": "5a7cc1ed48adcd54f318425f6a6caada100550d0",
          "url": "https://github.com/risingwavelabs/jsonbb/commit/e7c18e8b25f1a1072cca732ae6e030f00bf2fd62"
        },
        "date": 1700039163967,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_string/jsonbb",
            "value": 32,
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
            "value": 5899389,
            "range": "± 85526",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog parse/jsonbb",
            "value": 2841507,
            "range": "± 128221",
            "unit": "ns/iter"
          },
          {
            "name": "twitter parse/jsonbb",
            "value": 1901200,
            "range": "± 14987",
            "unit": "ns/iter"
          },
          {
            "name": "canada to_string/jsonbb",
            "value": 10484887,
            "range": "± 55739",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog to_string/jsonbb",
            "value": 2807906,
            "range": "± 20314",
            "unit": "ns/iter"
          },
          {
            "name": "twitter to_string/jsonbb",
            "value": 1612606,
            "range": "± 32509",
            "unit": "ns/iter"
          },
          {
            "name": "hash/jsonbb",
            "value": 215,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "eq/jsonbb",
            "value": 153,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cmp/jsonbb",
            "value": 160,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json[i]/jsonbb",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json['key']/jsonbb",
            "value": 66,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "[json['key'] for json in array]/jsonbb",
            "value": 39747,
            "range": "± 180",
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
            "value": 93,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->areaNames->205705994 index/jsonbb",
            "value": 131,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->topicNames->324846100 index/jsonbb",
            "value": 114,
            "range": "± 0",
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
            "value": 106,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "array_push/jsonbb",
            "value": 113,
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
          "id": "c270627b9374ec83094728e17634234ae0caee69",
          "message": "add doctest for `is_*` functions\n\nSigned-off-by: Runji Wang <wangrunji0408@163.com>",
          "timestamp": "2023-11-15T17:11:07+08:00",
          "tree_id": "56f77542cc68a36b01bce31ee6f1f990441b9b9f",
          "url": "https://github.com/risingwavelabs/jsonbb/commit/c270627b9374ec83094728e17634234ae0caee69"
        },
        "date": 1700039777558,
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
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "canada parse/jsonbb",
            "value": 5868996,
            "range": "± 19711",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog parse/jsonbb",
            "value": 2785540,
            "range": "± 132865",
            "unit": "ns/iter"
          },
          {
            "name": "twitter parse/jsonbb",
            "value": 1885797,
            "range": "± 19605",
            "unit": "ns/iter"
          },
          {
            "name": "canada to_string/jsonbb",
            "value": 10467041,
            "range": "± 22209",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog to_string/jsonbb",
            "value": 2816070,
            "range": "± 12662",
            "unit": "ns/iter"
          },
          {
            "name": "twitter to_string/jsonbb",
            "value": 1654645,
            "range": "± 25151",
            "unit": "ns/iter"
          },
          {
            "name": "hash/jsonbb",
            "value": 214,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "eq/jsonbb",
            "value": 154,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cmp/jsonbb",
            "value": 159,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json[i]/jsonbb",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json['key']/jsonbb",
            "value": 66,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "[json['key'] for json in array]/jsonbb",
            "value": 39557,
            "range": "± 295",
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
            "value": 93,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->areaNames->205705994 index/jsonbb",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->topicNames->324846100 index/jsonbb",
            "value": 112,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "twitter->search_metadata->max_id_str index/jsonbb",
            "value": 116,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json[path]/jsonbb",
            "value": 107,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "array_push/jsonbb",
            "value": 112,
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
          "id": "a8c65e2eb960bbe2c5aedf90d44b86e6ae6f2dcc",
          "message": "fix clippy\n\nSigned-off-by: Runji Wang <wangrunji0408@163.com>",
          "timestamp": "2023-11-16T13:10:09+08:00",
          "tree_id": "a8a24220baefeef06a0e37d5fa5f3f7d5865c487",
          "url": "https://github.com/risingwavelabs/jsonbb/commit/a8c65e2eb960bbe2c5aedf90d44b86e6ae6f2dcc"
        },
        "date": 1700111783353,
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "canada parse/jsonbb",
            "value": 5863853,
            "range": "± 28625",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog parse/jsonbb",
            "value": 2749554,
            "range": "± 17146",
            "unit": "ns/iter"
          },
          {
            "name": "twitter parse/jsonbb",
            "value": 1873559,
            "range": "± 22533",
            "unit": "ns/iter"
          },
          {
            "name": "canada to_string/jsonbb",
            "value": 10343516,
            "range": "± 25644",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog to_string/jsonbb",
            "value": 2815250,
            "range": "± 19338",
            "unit": "ns/iter"
          },
          {
            "name": "twitter to_string/jsonbb",
            "value": 1612753,
            "range": "± 34881",
            "unit": "ns/iter"
          },
          {
            "name": "hash/jsonbb",
            "value": 210,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "eq/jsonbb",
            "value": 156,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cmp/jsonbb",
            "value": 158,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json[i]/jsonbb",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json['key']/jsonbb",
            "value": 62,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "[json['key'] for json in array]/jsonbb",
            "value": 40276,
            "range": "± 227",
            "unit": "ns/iter"
          },
          {
            "name": "canada->type index/jsonbb",
            "value": 67,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->areaNames index/jsonbb",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->areaNames->205705994 index/jsonbb",
            "value": 129,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->topicNames->324846100 index/jsonbb",
            "value": 108,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "twitter->search_metadata->max_id_str index/jsonbb",
            "value": 114,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "json[path]/jsonbb",
            "value": 110,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "array_push/jsonbb",
            "value": 110,
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
          "id": "d9742a8d7794e9d8629666a6629bc2866650c1e2",
          "message": "add `NumberRef::to_number` and bump version to v0.1.3\n\nSigned-off-by: Runji Wang <wangrunji0408@163.com>",
          "timestamp": "2023-11-20T16:00:24+08:00",
          "tree_id": "92f64ff45ad521f26b8c6896753faa0abc9097de",
          "url": "https://github.com/risingwavelabs/jsonbb/commit/d9742a8d7794e9d8629666a6629bc2866650c1e2"
        },
        "date": 1700467546745,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_string/jsonbb",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "from_i64/jsonbb",
            "value": 33,
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
            "name": "citm_catalog parse/jsonbb",
            "value": 3262258,
            "range": "± 57400",
            "unit": "ns/iter"
          },
          {
            "name": "twitter parse/jsonbb",
            "value": 1880427,
            "range": "± 18897",
            "unit": "ns/iter"
          },
          {
            "name": "canada parse/jsonbb",
            "value": 4921977,
            "range": "± 68822",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog to_string/jsonbb",
            "value": 2800672,
            "range": "± 9690",
            "unit": "ns/iter"
          },
          {
            "name": "twitter to_string/jsonbb",
            "value": 1587028,
            "range": "± 17202",
            "unit": "ns/iter"
          },
          {
            "name": "canada to_string/jsonbb",
            "value": 10551829,
            "range": "± 29350",
            "unit": "ns/iter"
          },
          {
            "name": "hash/jsonbb",
            "value": 209,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "eq/jsonbb",
            "value": 155,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cmp/jsonbb",
            "value": 151,
            "range": "± 0",
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
            "value": 39785,
            "range": "± 113",
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
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->areaNames->205705994 index/jsonbb",
            "value": 130,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->topicNames->324846100 index/jsonbb",
            "value": 120,
            "range": "± 0",
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
            "value": 114,
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
          "id": "d0c88bf5a5edd73ad36ac8e02d107daed2372e53",
          "message": "compress numbers\n\nSigned-off-by: Runji Wang <wangrunji0408@163.com>",
          "timestamp": "2023-12-20T23:46:58+08:00",
          "tree_id": "617f10c2b3891b3be24f21eed6aeed08712f1854",
          "url": "https://github.com/risingwavelabs/jsonbb/commit/d0c88bf5a5edd73ad36ac8e02d107daed2372e53"
        },
        "date": 1703087552401,
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
            "value": 5862940,
            "range": "± 31407",
            "unit": "ns/iter"
          },
          {
            "name": "twitter parse/jsonbb",
            "value": 1895164,
            "range": "± 8754",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog parse/jsonbb",
            "value": 2811207,
            "range": "± 30410",
            "unit": "ns/iter"
          },
          {
            "name": "canada to_string/jsonbb",
            "value": 10294198,
            "range": "± 22517",
            "unit": "ns/iter"
          },
          {
            "name": "twitter to_string/jsonbb",
            "value": 1299706,
            "range": "± 11367",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog to_string/jsonbb",
            "value": 2201857,
            "range": "± 7150",
            "unit": "ns/iter"
          },
          {
            "name": "hash/jsonbb",
            "value": 223,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "eq/jsonbb",
            "value": 155,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cmp/jsonbb",
            "value": 159,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json[i]/jsonbb",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json['key']/jsonbb",
            "value": 66,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "[json['key'] for json in array]/jsonbb",
            "value": 39960,
            "range": "± 131",
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
            "value": 93,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->areaNames->205705994 index/jsonbb",
            "value": 132,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog->topicNames->324846100 index/jsonbb",
            "value": 112,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "twitter->search_metadata->max_id_str index/jsonbb",
            "value": 115,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "json[path]/jsonbb",
            "value": 105,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "array_push/jsonbb",
            "value": 115,
            "range": "± 2",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}