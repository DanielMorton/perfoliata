use serde_json::Value;
use std::collections::HashMap;
use crate::ClientError;

/// Flatten nested JSON objects (simplified version)
pub fn flatten_dict(obj: &Value) -> HashMap<String, Value> {
    let mut result = HashMap::new();

    if let Some(object) = obj.as_object() {
        for (key, value) in object {
            match value {
                Value::Object(_) => {
                    let nested = flatten_dict(value);
                    for (nested_key, nested_value) in nested {
                        result.insert(format!("{key}_{nested_key}"), nested_value);
                    }
                }
                _ => {
                    result.insert(key.clone(), value.clone());
                }
            }
        }
    }

    result
}

/// Convert a serde_json::Value to u32
pub fn json_value_to_u32(value: &Value) -> Result<u32, ClientError> {
    match value {
        Value::Number(n) => {
            if let Some(u) = n.as_u64() {
                u.try_into()
                    .map_err(|_| ClientError::Parse(format!("Value {} too large for u32", u)))
            } else if let Some(i) = n.as_i64() {
                if i >= 0 {
                    (i as u64).try_into()
                        .map_err(|_| ClientError::Parse(format!("Value {} too large for u32", i)))
                } else {
                    Err(ClientError::Parse(format!("Cannot convert negative value to u32: {}", i)))
                }
            } else {
                Err(ClientError::Parse("Not a valid integer".to_string()))
            }
        }
        Value::String(s) => {
            s.parse::<u32>()
                .map_err(|_| ClientError::Parse(format!("Failed to parse string as u32: '{}'", s)))
        }
        _ => Err(ClientError::Parse(format!("Expected number or string, got: {:?}", value)))
    }
}

/// Helper function to extract u32 from JSON object by field name
pub fn extract_u32_field(obj: &Value, field_name: &str) -> Result<u32, ClientError> {
    let value = obj.get(field_name)
        .ok_or_else(|| ClientError::MissingField(field_name.to_string()))?;

    json_value_to_u32(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_flatten_dict() {
        let nested_json = json!({
            "name": "test",
            "user": {
                "id": 123,
                "login": "testuser"
            }
        });

        let flattened = flatten_dict(&nested_json);

        assert_eq!(flattened.get("name"), Some(&json!("test")));
        assert_eq!(flattened.get("user_id"), Some(&json!(123)));
        assert_eq!(flattened.get("user_login"), Some(&json!("testuser")));
    }
}
