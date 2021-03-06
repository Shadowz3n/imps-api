use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq)]
pub struct CustomDateTime(pub NaiveDateTime);

impl Serialize for CustomDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = self.0.format("%Y-%m-%dT%H:%M:%S.%3fZ");
        serializer.serialize_str(&s.to_string())
    }
}