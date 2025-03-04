use serde::{Serialize, Deserialize};


pub fn serialize<T: Serialize>(value: &T) -> String {
  serde_json::to_string(&value).unwrap()
}

pub fn deserialize<'a, T: Deserialize<'a>>(json: &'a str) -> T {
  serde_json::from_str(json).unwrap()
}
