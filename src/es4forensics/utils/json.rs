use serde_json::Value;

pub fn add_to_json(value: &Value, ident: &str, s: Value) -> Value {
    match value {
        Value::Object(m) => {
            let mut m = m.clone();
            m.insert(ident.to_string(), s);
            Value::Object(m)
        }
        v => v.clone(),
    }
}
