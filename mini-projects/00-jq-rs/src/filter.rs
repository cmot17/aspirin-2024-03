use crate::filter::FilterError::InvalidFilter;
use crate::parse::{DelTarget, Operation};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FilterError {
    #[error("Invalid filter: {0}")]
    InvalidFilter(String),
    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Addition type mismatch error: {0}")]
    TypeError(String),
    #[error("Expected an array to perform addition: {0}")]
    AdditionError(String),
}

pub fn filter(
    input: serde_json::Value,
    filters: Vec<Operation>,
) -> Result<Vec<serde_json::Value>, FilterError> {
    let mut results = vec![input];

    for operation in filters {
        let mut new_results = Vec::new();
        for value in results {
            match operation {
                Operation::Add => {
                    new_results.push(add_value(value)?);
                }
                Operation::Del(ref target) => {
                    new_results.push(delete_value(value, target)?);
                }
                Operation::Len => {
                    new_results.push(get_value_length(value));
                }
                Operation::Slice(start, end) => {
                    new_results.push(slice_value(&value, start, end)?);
                }
                Operation::Iterate => {
                    if let Value::Array(arr) = value {
                        for item in arr {
                            new_results.push(item.clone());
                        }
                    } else {
                        return Err(InvalidFilter(
                            "Cannot iterate over non-array value".to_string(),
                        ));
                    }
                }

                Operation::Identity => {
                    new_results.push(value);
                }
                Operation::Object(ref key) => {
                    new_results.push(apply_object_operation(value, key.as_str())?);
                }
                Operation::Index(index) => {
                    new_results.push(apply_index_operation(value, index)?);
                }
            }
        }
        results = new_results;
    }
    Ok(results)
}

fn apply_index_operation(value: Value, index: usize) -> Result<Value, FilterError> {
    match value {
        Value::Array(arr) => arr.get(index).cloned().ok_or_else(|| {
            FilterError::InvalidFilter(format!("Array index {} is out of bounds", index))
        }),
        _ => Err(FilterError::InvalidFilter(
            "Index operation can only be applied to arrays".into(),
        )),
    }
}

fn apply_object_operation(value: Value, key: &str) -> Result<Value, FilterError> {
    match value {
        Value::Object(map) => Ok(map.get(key).cloned().unwrap_or(Value::Null)),
        _ => Err(FilterError::InvalidFilter(
            "Object operation can only be applied to objects".into(),
        )),
    }
}

fn slice_value(
    value: &Value,
    start: Option<usize>,
    end: Option<usize>,
) -> Result<Value, FilterError> {
    fn get_slice_indices(
        len: usize,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<(usize, usize), FilterError> {
        let start_idx = start.unwrap_or(0);
        let end_idx = end.unwrap_or(len);

        if start_idx > end_idx || end_idx > len {
            Err(InvalidFilter("Invalid slice indices".to_string()))
        } else {
            Ok((start_idx, end_idx))
        }
    }

    match value {
        Value::Array(arr) => {
            let (start_idx, end_idx) = get_slice_indices(arr.len(), start, end)?;
            Ok(Value::Array(arr[start_idx..end_idx].to_vec()))
        }
        Value::String(s) => {
            let (start_idx, end_idx) = get_slice_indices(s.chars().count(), start, end)?;
            let sliced_str: String = s
                .chars()
                .skip(start_idx)
                .take(end_idx - start_idx)
                .collect();
            Ok(Value::String(sliced_str))
        }
        _ => Err(InvalidFilter(
            "Slice operation can only be applied to arrays and strings".to_string(),
        )),
    }
}

fn get_value_length(value: Value) -> Value {
    Value::Number(serde_json::Number::from(match value {
        Value::Null => 0,
        Value::Bool(_) | Value::Number(_) => 1,
        Value::String(s) => s.len(),
        Value::Array(arr) => arr.len(),
        Value::Object(obj) => obj.len(),
    }))
}

fn delete_value(value: Value, target: &DelTarget) -> Result<Value, FilterError> {
    match (value, target) {
        (Value::Object(mut map), DelTarget::Object(key)) => {
            map.remove(key);
            Ok(Value::Object(map))
        }
        (Value::Array(mut vec), DelTarget::ArrayIndexes(indices)) => {
            // Clone the indices to work with a local owned copy
            let mut indices = indices.clone();

            // Sort indices in reverse order to avoid shifting issues
            indices.sort_unstable_by(|a, b| b.cmp(a));

            for &index in &indices {
                if index < vec.len() {
                    vec.remove(index);
                } else {
                    return Err(FilterError::InvalidFilter(format!(
                        "Array index out of bounds: {}",
                        index
                    )));
                }
            }
            Ok(Value::Array(vec))
        }
        (Value::Object(_), DelTarget::ArrayIndexes(_)) => Err(FilterError::InvalidFilter(
            "Cannot delete index from non-array value".to_string(),
        )),
        (Value::Array(_), DelTarget::Object(_)) => Err(FilterError::InvalidFilter(
            "Cannot delete key from non-object value".to_string(),
        )),
        (_, _) => Err(FilterError::InvalidFilter(
            "Incorrect value or target type".to_string(),
        )),
    }
}

fn add_value(result: serde_json::Value) -> Result<serde_json::Value, FilterError> {
    if let serde_json::Value::Array(values) = result {
        let mut sum = serde_json::Value::Null;
        for value in values {
            sum = match (sum, value) {
                (serde_json::Value::Null, v) => v,
                (serde_json::Value::Number(n1), serde_json::Value::Number(n2)) => {
                    json!(n1.as_f64().unwrap() + n2.as_f64().unwrap())
                }
                (serde_json::Value::String(s1), serde_json::Value::String(s2)) => {
                    json!(s1 + &s2)
                }
                _ => {
                    return Err(FilterError::TypeError(
                        "Cannot add number and string".to_string(),
                    ))
                }
            };
        }
        Ok(sum)
    } else {
        Err(FilterError::AdditionError(
            "Input must be an array for Add operation".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_index_on_array() {
        let input = json!([10, 20, 30]);
        let index = 1;
        let result = apply_index_operation(input, index).unwrap();
        assert_eq!(result, json!(20));
    }

    #[test]
    fn test_apply_index_on_empty_array() {
        let input = json!([]);
        let index = 0;
        let result = apply_index_operation(input, index);
        assert!(matches!(result, Err(FilterError::InvalidFilter(_))));
    }

    #[test]
    fn test_index_out_of_bounds() {
        let input = json!([1, 2, 3]);
        let index = 3; // Out of bounds
        let result = apply_index_operation(input, index);
        assert!(matches!(result, Err(FilterError::InvalidFilter(_))));
    }

    #[test]
    fn test_apply_index_on_object() {
        let input = json!({"a": 1, "b": 2});
        let index = 0;
        let result = apply_index_operation(input, index);
        assert!(matches!(result, Err(FilterError::InvalidFilter(_))));
    }

    #[test]
    fn test_apply_index_on_string() {
        let input = json!("test");
        let index = 0;
        let result = apply_index_operation(input, index);
        assert!(matches!(result, Err(FilterError::InvalidFilter(_))));
    }

    #[test]
    fn test_apply_index_on_number() {
        let input = json!(100);
        let index = 0;
        let result = apply_index_operation(input, index);
        assert!(matches!(result, Err(FilterError::InvalidFilter(_))));
    }

    #[test]
    fn test_apply_index_on_null() {
        let input = json!(null);
        let index = 0;
        let result = apply_index_operation(input, index);
        assert!(matches!(result, Err(FilterError::InvalidFilter(_))));
    }

    #[test]
    fn test_apply_index_on_boolean() {
        let input = json!(true);
        let index = 0;
        let result = apply_index_operation(input, index);
        assert!(matches!(result, Err(FilterError::InvalidFilter(_))));
    }

    #[test]
    fn test_apply_index_at_boundary() {
        let input = json!([5, 6, 7]);
        let index = 2; // Boundary case
        let result = apply_index_operation(input, index).unwrap();
        assert_eq!(result, json!(7));
    }

    #[test]
    fn test_iterate_operation() {
        let input = json!([{"name": "Leo Lightning"}, {"name": "Maximus Defender"}, {"name": "Sophie Swift"}]);
        let filters = vec![Operation::Iterate, Operation::Object("name".to_string())];
        let result = filter(input, filters).unwrap();
        assert_eq!(
            result,
            vec![
                json!("Leo Lightning"),
                json!("Maximus Defender"),
                json!("Sophie Swift")
            ]
        );
    }
    #[test]
    fn test_slice_array() {
        let input = json!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let filters = vec![Operation::Slice(Some(2), Some(5))];
        let result = filter(input, filters).unwrap();
        assert_eq!(result, vec![json!([2, 3, 4])]);
    }

    #[test]
    fn test_slice_string() {
        let input = json!("hello world");
        let filters = vec![Operation::Slice(Some(6), Some(11))];
        let result = filter(input, filters).unwrap();
        assert_eq!(result, vec![json!("world")]);
    }
    #[test]
    fn test_del_object_key() {
        let input = json!({"fizz": "buzz", "fizzes": "buzzes", "baz": null, "fuzz": true, "bizz": 22.0, "biz": 42});
        let filters = vec![Operation::Del(DelTarget::Object("fizzes".to_string()))];
        let result = filter(input, filters).unwrap();
        assert_eq!(
            result,
            vec![json!({"fizz": "buzz", "baz": null, "fuzz": true, "bizz": 22.0, "biz": 42})]
        );
    }

    #[test]
    fn test_del_array_indices() {
        let input = json!([500, 5, 2, 300, 40, 1000]);
        let filters = vec![Operation::Del(DelTarget::ArrayIndexes(vec![1, 3, 4]))];
        let result = filter(input, filters).unwrap();
        assert_eq!(result, vec![json!([500, 2, 1000])]);
    }

    #[test]
    fn test_del_invalid_object_key() {
        let input = json!([1, 2, 3]);
        let filters = vec![Operation::Del(DelTarget::Object("a".to_string()))];
        let result = filter(input, filters);
        assert!(matches!(result, Err(FilterError::InvalidFilter(_))));
    }

    #[test]
    fn test_del_invalid_array_index() {
        let input = json!({"a": 1, "b": 2});
        let filters = vec![Operation::Del(DelTarget::ArrayIndexes(vec![0]))];
        let result = filter(input, filters);
        assert!(matches!(result, Err(FilterError::InvalidFilter(_))));
    }
    #[test]
    fn test_add_str() {
        let input = json!(["one", "two", "three"]);
        let filters = vec![Operation::Identity, Operation::Add];
        let result = filter(input, filters).unwrap();
        assert_eq!(result, vec![json!("onetwothree")]);
    }

    #[test]
    fn test_add_num() {
        let input = json!([1, 2, 3]);
        let filters = vec![Operation::Identity, Operation::Add];
        let result = filter(input, filters).unwrap();
        assert_eq!(result, vec![json!(6.0)]);
    }

    #[test]
    fn test_add_mixed() {
        let input = json!([1, 2, "three"]);
        let filters = vec![Operation::Identity, Operation::Add];
        let result = filter(input, filters);
        assert!(matches!(result, Err(FilterError::TypeError(_))));
    }

    #[test]
    fn test_len_object() {
        let input = json!({"a": 1, "b": 2, "c": 3});
        let filters = vec![Operation::Len];
        let result = filter(input, filters).unwrap();
        assert_eq!(result, vec![json!(3)]);
    }

    #[test]
    fn test_len_array() {
        let input = json!([1, 2, 3, 4, 5]);
        let filters = vec![Operation::Len];
        let result = filter(input, filters).unwrap();
        assert_eq!(result, vec![json!(5)]);
    }

    #[test]
    fn test_len_string() {
        let input = json!("hello");
        let filters = vec![Operation::Len];
        let result = filter(input, filters).unwrap();
        assert_eq!(result, vec![json!(5)]);
    }

    #[test]
    fn test_len_number() {
        let input = json!(42);
        let filters = vec![Operation::Len];
        let result = filter(input, filters).unwrap();
        assert_eq!(result, vec![json!(1)]);
    }

    #[test]
    fn test_len_null() {
        let input = json!(null);
        let filters = vec![Operation::Len];
        let result = filter(input, filters).unwrap();
        assert_eq!(result, vec![json!(0)]);
    }

    #[test]
    fn test_len_boolean() {
        let input = json!(true);
        let filters = vec![Operation::Len];
        let result = filter(input, filters).unwrap();
        assert_eq!(result, vec![json!(1)]);
    }
}
