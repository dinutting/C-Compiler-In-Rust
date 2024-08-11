pub use json::*;

pub fn load_tokens() -> JsonValue {
    json::parse(r#"
    {
      "tokens": [
        {
        "tokenType": "whitespace",
        "regex": "\\s"
        },
        {
        "tokenType": "number",
        "regex": "\\d"
        }
      ]
    }"#).unwrap()
}