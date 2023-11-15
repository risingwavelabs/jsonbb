/// Construct a `jsonbb::Value` from a JSON literal.
///
/// ```
/// # use jsonbb::json;
/// #
/// let value = json!({
///     "code": 200,
///     "success": true,
///     "payload": {
///         "features": [
///             "serde",
///             "json"
///         ],
///         "homepage": null
///     }
/// });
/// ```
#[macro_export(local_inner_macros)]
macro_rules! json {
    ($($json:tt)+) => {
        jsonbb::Value::from(jsonbb::serde_json::json!($($json)+))
    };
}
