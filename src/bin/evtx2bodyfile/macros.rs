macro_rules! from_json {
    ($value: ident, $( $att:expr ),+ ) => {
        {
            let mut value = $value;
            $(
                value = value.get($att).ok_or(anyhow::anyhow!("missing '{}' key in {}", $att, value))?;
            )+
            value
        }
    };
}

pub (crate) use from_json;