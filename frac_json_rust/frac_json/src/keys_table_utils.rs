use std::collections::HashMap;

use serde_json::Value;

use crate::keys_table::{KeysTable, KeysTableLocation};

pub fn global_table_from_keys(keys: Vec<String>) -> KeysTable {
    KeysTable::new(keys, KeysTableLocation::Global)
}

pub fn global_table_from_json(json: &Value) -> KeysTable {
    global_table_from_json_limited(json, 0, 0)
}

pub fn global_table_from_json_limited(
    json: &Value,
    max_count: usize,
    occurrence_cutoff: usize,
) -> KeysTable {
    let mut key_usages: HashMap<&String, usize> = HashMap::new();
    let mut pending_objects: Vec<&Value> = vec![json];
    while pending_objects.len() > 0 {
        let value = pending_objects.pop();
        match value {
            Some(value) => match value {
                Value::Array(array) => pending_objects.extend(array),
                Value::Object(object) => {
                    for (k, v) in object {
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

    let keys: Vec<String> = key_usages.map(|(k, _v)| k.to_string()).collect();
    return global_table_from_keys(keys);
}
