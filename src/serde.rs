use serde::{Serializer};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn serialize_dt<S>(date: &chrono::DateTime<chrono::Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}