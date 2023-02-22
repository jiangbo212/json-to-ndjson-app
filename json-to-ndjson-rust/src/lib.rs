use serde_json::{from_str, Value, to_string};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn json_to_ndjson(json_str: &str) -> String {
    let values: Vec<Value> = from_str(json_str).expect("不是一个合法的jsonArray对象");
    let mut ndjson_str = String::new();
    for value in values {
        let json_str = serde_json::to_string(&value).expect("不是一个合法json字符串");
        ndjson_str.push_str(&json_str);
        ndjson_str.push('\n');
    }
    ndjson_str
}

#[wasm_bindgen]
pub fn ndjson_to_json(ndjson_str: &str) -> String {
    let mut values = Vec::new();
    for line in ndjson_str.lines() {
        let value: Value = serde_json::from_str(line).expect("第几行不是一个合法json字符串");
        values.push(value);
    }

    let json_str = to_string(&values).expect("序列化一个json array异常");
    json_str
}

#[cfg(test)]
mod tests {
    use crate::{json_to_ndjson, ndjson_to_json};


    #[test]
    fn test_json_to_ndjson() {
        let json_str = r#"[{"name":"Alice","age":25},{"name":"Bob","age":30}]"#;
        let ndjson_str = json_to_ndjson(json_str);
        println!("{}", ndjson_str);
        let asset_right = r#"{"age":25,"name":"Alice"}
{"age":30,"name":"Bob"}
"#;
        assert_eq!(ndjson_str, asset_right);
    }

    #[test]
    fn test_ndjson_to_json() {
        let ndjson_str = r#"{"age":25,"name":"Alice"}
{"age":30,"name":"Bob"}
"#;
        let json_str = ndjson_to_json(ndjson_str);
        println!("{}", json_str);
        let assert_right = r#"[{"age":25,"name":"Alice"},{"age":30,"name":"Bob"}]"#;
        assert_eq!(json_str, assert_right);
    }
}
