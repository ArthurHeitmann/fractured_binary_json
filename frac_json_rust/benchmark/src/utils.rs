use std::path::Path;

use serde_json::Value;

fn find_res_folder() -> String {
    let path = std::env::current_exe().unwrap();
    let mut folder = path.parent().unwrap();
    for _ in 0..5 {
        let files = std::fs::read_dir(folder).unwrap();
        for file in files {
            let file = file.unwrap();
            let file_path = file.path();
            let name = file.file_name();
            let name = name.to_str().unwrap();
            let parent = file_path
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap();
            if name == "res" {
                return file.path().to_str().unwrap().to_string();
            }
            if name == "benchmark" && !parent.ends_with("target") {
                return Path::new(&file_path)
                    .join("res")
                    .to_str()
                    .unwrap()
                    .to_string();
            }
        }
        folder = folder.parent().unwrap();
    }
    panic!("benchmark/res/ folder not found")
}

pub fn get_test_files(file_names: &[&str]) -> Vec<(String, Value)> {
    let res_folder = find_res_folder();
    let test_files = file_names
        .iter()
        .map(|file_name| {
            Path::new(&res_folder)
                .join(file_name)
                .to_str()
                .unwrap()
                .to_string()
        })
        .map(|file_path| {
            let bytes = if file_path.ends_with(".zst") {
                let compressed_bytes_reader = std::fs::File::open(&file_path).unwrap();
                zstd::decode_all(compressed_bytes_reader).unwrap()
            } else {
                std::fs::read(&file_path).unwrap()
            };
            let decompressed = String::from_utf8(bytes).unwrap();
            let value = serde_json::from_str(&decompressed).unwrap();
            (file_path.to_string(), value)
        });
    Vec::from_iter(test_files)
}

const FALLBACK_KEYS: &[&str] = &["statuses"];
pub fn get_test_files_as_array(file_names: &[&str]) -> Vec<(String, Vec<Value>)> {
    let files = get_test_files(file_names);
    files
        .into_iter()
        .map(|(path, value)| {
            let array = match value {
                Value::Array(array) => array,
                Value::Object(object) => {
                    for key in FALLBACK_KEYS {
                        if let Some(array) = object.get(*key) {
                            if let Value::Array(array) = array {
                                return (path, array.clone());
                            }
                        }
                    }
                    panic!("Expected array in file {}", path);
                }
                _ => panic!("Expected array in file {}, got {}", path, value),
            };
            (path, array)
        })
        .collect()
}

pub fn get_size_unit(size: usize) -> (f64, &'static str) {
    if size < 1024 {
        (1.0, "B")
    } else if size < 1024 * 1024 {
        (1024.0, "KB")
    } else {
        (1024.0 * 1024.0, "MB")
    }
}

pub fn json_eq(a: &Value, b: &Value) {
    // assert_eq!(
    //     serde_json::to_string(a).unwrap(),
    //     serde_json::to_string(b).unwrap()
    // );
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => {
            assert_eq!(a.len(), b.len());
            for (key, value) in a {
                assert!(b.contains_key(key));
                json_eq(value, &b[key]);
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            assert_eq!(a.len(), b.len());
            for (i, value) in a.iter().enumerate() {
                json_eq(value, &b[i]);
            }
        }
        (Value::String(a), Value::String(b)) => assert_eq!(a, b),
        (Value::Number(a), Value::Number(b)) => {
            if a.is_f64() || b.is_f64() {
                assert!(float_rel_eq(
                    a.as_f64().unwrap(),
                    b.as_f64().unwrap(),
                    0.000001
                ))
            } else {
                assert_eq!(a.as_i64().unwrap(), b.as_i64().unwrap())
            }
        }
        (Value::Bool(a), Value::Bool(b)) => assert_eq!(a, b),
        (Value::Null, Value::Null) => {}
        _ => panic!("JSON values are not equal: {:?} != {:?}", a, b),
    }
}

fn float_rel_eq(a: f64, b: f64, epsilon: f64) -> bool {
    let diff = (a - b).abs();
    let max = a.abs().max(b.abs());
    diff < epsilon * max
}

pub struct Table {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn to_markdown(self: &Self) -> String {
        let mut max_col_widths = vec![0; self.header.len()];
        for row in self.rows.iter().chain(std::iter::once(&self.header)) {
            for (i, cell) in row.iter().enumerate() {
                max_col_widths[i] = max_col_widths[i].max(cell.len());
            }
        }
        let mut table_str = String::new();
        // Header
        table_str.push_str("|");
        for (i, cell) in self.header.iter().enumerate() {
            table_str.push_str(&format!(" {:<1$} |", cell, max_col_widths[i]));
        }
        table_str.push_str("\n");
        // Separator
        table_str.push_str("|");
        for width in &max_col_widths {
            table_str.push_str(&format!("-{:-<1$}-|", "", width));
        }
        table_str.push_str("\n");
        // Rows
        for row in &self.rows {
            table_str.push_str("|");
            for (i, cell) in row.iter().enumerate() {
                table_str.push_str(&format!(" {:<1$} |", cell, max_col_widths[i]));
            }
            table_str.push_str("\n");
        }
        table_str
    }

    pub fn to_csv(self: &Self) -> String {
        let mut table_str = String::new();
        // Header
        table_str.push_str(&self.header.join(","));
        table_str.push_str("\n");
        // Rows
        for row in &self.rows {
            table_str.push_str(&row.join(","));
            table_str.push_str("\n");
        }
        table_str
    }
}
