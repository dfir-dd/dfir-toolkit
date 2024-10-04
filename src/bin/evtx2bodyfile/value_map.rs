use flow_record::{
    prelude::{rmpv, FieldType},
    ToMsgPackValue,
};

pub struct ValueMap<'v>(&'v serde_json::Value);

impl<'v> From<&'v serde_json::Value> for ValueMap<'v> {
    fn from(value: &'v serde_json::Value) -> Self {
        Self(value)
    }
}

impl<'v> ToMsgPackValue for ValueMap<'v> {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::Array(vec![json_to_msgpack(self.0)])
    }

    fn field_type() -> FieldType {
        FieldType::DictList
    }
}

fn json_to_msgpack(v: &serde_json::Value) -> rmpv::Value {
    match v {
        serde_json::Value::Null => rmpv::Value::Nil,
        serde_json::Value::Bool(b) => rmpv::Value::Boolean(*b),
        serde_json::Value::Number(n) => rmpv::Value::String(n.to_string().into()),
        serde_json::Value::String(s) => rmpv::Value::String(s.to_string().into()),
        serde_json::Value::Array(vec) => {
            rmpv::Value::Array(vec.iter().map(json_to_msgpack).collect())
        }
        serde_json::Value::Object(map) => rmpv::Value::Map(
            map.into_iter()
                .map(|(k, v)| {
                    (
                        rmpv::Value::String(k.to_string().into()),
                        json_to_msgpack(v),
                    )
                })
                .collect(),
        ),
    }
}
