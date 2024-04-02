use std::collections::HashMap;

use serde_json::Value;

use crate::keys_table::{GlobalKeysTable, MAX_KEY_LENGTH, MAX_TABLE_SIZE};

pub fn global_table_from_keys(keys: Vec<String>) -> Result<Vec<u8>, String> {
    let table = GlobalKeysTable::new(keys);
    let mut bytes: Vec<u8> = Vec::new();
    table.write_keys_table(&mut bytes)?;
    return Ok(bytes);
}

pub fn global_table_from_json(json: &Value) -> Result<Vec<u8>, String> {
    global_table_from_json_limited(json, None, None)
}

pub fn global_table_from_json_limited(
    json: &Value,
    max_count: Option<usize>,
    occurrence_cutoff: Option<usize>,
) -> Result<Vec<u8>, String> {
    let max_count = max_count.unwrap_or(MAX_TABLE_SIZE);
    let occurrence_cutoff = occurrence_cutoff.unwrap_or(1);
    if max_count > MAX_TABLE_SIZE {
        return Err(format!(
            "max_count {} is greater than MAX_GLOBAL_TABLE_SIZE {}",
            max_count, MAX_TABLE_SIZE
        ));
    }
    let mut key_usages: HashMap<&String, usize> = HashMap::new();
    let mut pending_objects: Vec<&Value> = vec![json];
    while pending_objects.len() > 0 {
        let value = pending_objects.pop();
        match value {
            Some(value) => match value {
                Value::Array(array) => pending_objects.extend(array),
                Value::Object(object) => {
                    for (k, v) in object {
                        if k.len() > MAX_KEY_LENGTH {
                            continue;
                        }
                        key_usages
                            .entry(k)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                        pending_objects.push(v);
                    }
                }
                _ => (),
            },
            None => break,
        }
    }

    let mut key_usages: Vec<(&String, usize)> = key_usages.iter().map(|(k, v)| (*k, *v)).collect();
    key_usages.sort_by(|a, b| b.1.cmp(&a.1));

    let key_usages = if max_count > 0 {
        key_usages.iter().take(max_count)
    } else {
        key_usages.iter().take(usize::MAX)
    };
    let key_usages = key_usages.filter(|(_k, v)| *v >= occurrence_cutoff);

    let keys: Vec<String> = key_usages.map(|(k, _v)| (*k).clone()).collect();
    return global_table_from_keys(keys);
}
