use jsonbb::{from_value, json};
use jsonbb_path::JsonPath;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    pub path: JsonPath,
}

#[test]
fn can_deserialize_json_path() {
    let config_json = json!({ "path": "$.foo.*" });
    let config = from_value::<Config>(config_json).expect("deserializes");
    let value = json!({"foo": [1, 2, 3]});
    let nodes = config.path.query(value.as_ref()).all();
    assert_eq!(nodes, vec![1, 2, 3]);
}
