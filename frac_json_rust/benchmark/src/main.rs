#[macro_use]
extern crate approx;
use std::{io::{stdout, Read, Write}, path::Path, time::Duration};
use frac_json::ByteStream;
use serde_json::Value;
use zstd::bulk::{compress, decompress};

const TEST_FILE_NAMES: [&str; 11] = [
    "MediaContent.json.zst",
    "reddit_comments_1.json.zst",
    "reddit_posts_1.json.zst",
    "TwitterTimeline.json.zst",
    "CouchDB4k.json.zst",
    "reddit_comments_100.json.zst",
    "reddit_posts_100.json.zst",
    "twitter.json.zst",
    "citm_catalog.json.zst",
    "canada.json.zst",
    "jeopardy.json.zst",
];

const TEST_FUNCTION_TEMPLATES: [(&str, fn(&Value, bool) -> (Vec<u8>, Duration), fn(&Vec<u8>, bool) -> (Value, Duration)); 7] = [
    ("plain text", encode_json, decode_json),
    ("UBJSON", encode_ubjson, decode_ubjson),
    ("MessagePack", encode_messagepack, decode_messagepack),
    ("Smile", encode_smile, decode_smile),
    ("Smile (+shared)", encode_smile_shared, decode_smile),
    ("fracture json", encode_frac_json, decode_frac_json),
    ("fracture json (+global table)", encode_frac_json_global_keys_table, decode_frac_json),
];

fn main() {
    benchmark_size_relative();
}

fn benchmark_size_relative() -> Vec<Vec<String>> {
    let test_functions: Vec<(&str, fn(&Value, bool) -> (Vec<u8>, Duration), fn(&Vec<u8>, bool) -> (Value, Duration), bool)> = TEST_FUNCTION_TEMPLATES
        .iter()
        .map(|(name, encode, decode)| (*name, *encode, *decode, false))
        .chain(
            TEST_FUNCTION_TEMPLATES
                .iter()
                .map(|(name, encode, decode)| (*name, *encode, *decode, true))
                .into_iter(),
        )
        .collect();
    let test_files = get_test_files();
    let mut results: Vec<Vec<String>> = Vec::new();
    
    let mut plain_text_sizes: Vec<usize> = Vec::new();
    for (_, value) in test_files.iter() {
        let size = serde_json::to_string(&value).unwrap().len();
        plain_text_sizes.push(size);
    }

    let mut longest_log_line = 0;
    for (i_func, (name, encode, decode, use_compression)) in test_functions.iter().enumerate() {
        let log_line = format!("{} (zstd: {:?})", name, use_compression);
        longest_log_line = longest_log_line.max(log_line.len());
        print!("\r{}{}", log_line, " ".repeat(longest_log_line - log_line.len()));
        stdout().flush().unwrap();
        
        let mut row: Vec<String> = Vec::new();
        let name = if *use_compression {
            format!("{} (+zstd)", name)
        } else {
            name.to_string()
        };
        row.push(name);
        for (i_file, (_, value)) in test_files.iter().enumerate() {
            let (encoded, _) = encode(&value, *use_compression);
            let (decoded, _) = decode(&encoded, *use_compression);
            assert_eq!(value, &decoded);
            let size = encoded.len();
            let plain_text_size = plain_text_sizes[i_file];
            let ratio = size as f64 / plain_text_size as f64 * 100.0;
            let mut ratio = format!("{:.1}%", ratio);
            if i_func == 0 {
                let (size_div, size_unit) = get_size_unit(plain_text_size);
                ratio = format!("{} ({:.1} {})", ratio, plain_text_size as f64 / size_div, size_unit);
            }
            row.push(ratio);
        }
        results.push(row);
    }
    println!();

    return results;
    // let mut column_units: Vec<(f64, &str)> = Vec::new();
    // let mut table_rel: Vec<Vec<String>> = Vec::new();
    // let mut table_abs: Vec<Vec<String>> = Vec::new();
    // let mut table_time: Vec<Vec<String>> = Vec::new();
    // let mut header: Vec<String> = Vec::new();
    // header.push("Format".to_string());
    // header.push("zstd".to_string());
    // for (i_file, (file_path, _)) in test_files.iter().enumerate() {
    //     let column_sizes = results
    //         .iter()
    //         .map(|r| r.1[i_file].0)
    //         .collect::<Vec<usize>>();
    //     column_units.push(get_sizes_unit(column_sizes));
    //     let file_name = Path::new(&file_path).file_name().unwrap();
    //     let file_name = file_name.to_str().unwrap().to_string();
    //     header.push(format!("{} [{}]", file_name, column_units[i_file].1));
    // }
    // table_rel.push(header.clone());
    // table_abs.push(header.clone());
    // table_time.push(header.clone());
    // for (i_res, (name, format_results)) in results.iter().enumerate() {
    //     let mut row_rel: Vec<String> = Vec::new();
    //     let mut row_abs: Vec<String> = Vec::new();
    //     let mut row_time: Vec<String> = Vec::new();
    //     row_rel.push(name.to_string());
    //     row_abs.push(name.to_string());
    //     row_time.push(name.to_string());
    //     row_rel.push(if test_functions[i_res].2 { "y" } else { "n" }.to_string());
    //     row_abs.push(if test_functions[i_res].2 { "y" } else { "n" }.to_string());
    //     row_time.push(if test_functions[i_res].2 { "y" } else { "n" }.to_string());
    //     for (i_file, (size, time)) in format_results.iter().enumerate() {
    //         row_rel.push(format!("{:.3}", size.1));
    //         let (size_div, unit) = column_units[i_file];
    //         if unit == "B" {
    //             row_abs.push(format!("{}", size.0));
    //         } else {
    //             row_abs.push(format!("{:.2}", size.0 as f64 / size_div));
    //         }
    //         row_time.push(format!("{:.2}", size.1));
    //     }
    //     table_rel.push(row_rel);
    //     table_abs.push(row_abs);
    //     table_time.push(row_time);
    // }

    // for row in table_rel {
    //     println!("{}", row.join(","));
    // }
    // println!();
    // for row in table_abs {
    //     println!("{}", row.join(","));
    // }
    // println!();
    // for row in table_time {
    //     println!("{}", row.join(","));
    // }

}

fn measure<T, F: Fn() -> T>(function: F) -> (T, Duration) {
    let start = std::time::Instant::now();
    let value = function();
    let elapsed = start.elapsed();
    (value, elapsed)
}

fn optionally_compress(data: &[u8], use_compression: bool) -> Vec<u8> {
    if use_compression {
        let compressed = compress(data, 3).unwrap();
        compressed
    } else {
        data.to_vec()
    }
}

fn optionally_decompress(data: &[u8], uses_compression: bool) -> Vec<u8> {
    if uses_compression {
        let buffer_size = data.len() * 50;
        let decompressed = decompress(data, buffer_size).unwrap();
        decompressed
    } else {
        data.to_vec()
    }
}

fn encode_json(value: &Value, compress: bool) -> (Vec<u8>, Duration) {
    measure(|| {
        let str = serde_json::to_string(value).unwrap();
        let bytes = str.as_bytes();
        optionally_compress(bytes, compress)
    })
}

fn decode_json(bytes: &Vec<u8>, uses_compression: bool) -> (Value, Duration) {
    measure(|| {
        let bytes = optionally_decompress(bytes, uses_compression);
        let str = std::str::from_utf8(&bytes).unwrap();
        let value = serde_json::from_str(str).unwrap();
        value
    })
}

fn encode_smile(value: &impl serde::ser::Serialize, compress: bool) -> (Vec<u8>, Duration) {
    let mut serializer = serde_smile::ser::Serializer::builder();
    let serializer = serializer
        .shared_properties(false)
        .shared_strings(false);
    measure(|| {
        let writer = Vec::new();
        let mut ser = serializer.build(writer);
        value.serialize(&mut ser).unwrap();
        let bytes = ser.into_inner();
        optionally_compress(&bytes, compress)
    })
}

fn decode_smile(bytes: &Vec<u8>, uses_compression: bool) -> (Value, Duration) {
    measure(|| {
        let bytes = optionally_decompress(bytes, uses_compression);
        serde_smile::from_slice(&bytes).unwrap()
    })
}

fn encode_smile_shared(value: &impl serde::ser::Serialize, compress: bool) -> (Vec<u8>, Duration) {
    let mut serializer = serde_smile::ser::Serializer::builder();
    let serializer = serializer
        .shared_properties(true)
        .shared_strings(true);
    measure(|| {
        let writer = Vec::new();
        let mut ser = serializer.build(writer);
        value.serialize(&mut ser).unwrap();
        let bytes = ser.into_inner();
        optionally_compress(&bytes, compress)
    })
}

fn encode_messagepack(value: &Value, compress: bool) -> (Vec<u8>, Duration) {
    measure(|| {
        let bytes = rmp_serde::to_vec(value).unwrap();
        optionally_compress(&bytes, compress)
    })
}

fn decode_messagepack(bytes: &Vec<u8>, uses_compression: bool) -> (Value, Duration) {
    measure(|| {
        let bytes = optionally_decompress(bytes, uses_compression);
        rmp_serde::from_slice(&bytes).unwrap()
    })
}

fn encode_ubjson(value: &Value, compress: bool) -> (Vec<u8>, Duration) {
    measure(|| {
        let bytes = serde_ub_json::to_bytes(value).unwrap();
        optionally_compress(&bytes, compress)
    })
}

fn decode_ubjson(bytes: &Vec<u8>, uses_compression: bool) -> (Value, Duration) {
    measure(|| {
        let bytes = optionally_decompress(bytes, uses_compression);
        serde_ub_json::from_bytes(&bytes[..]).unwrap()
    })
}

fn encode_frac_json(value: &Value, compress: bool) -> (Vec<u8>, Duration) {
    let compression = if compress { Some(3) } else { None };
    measure(|| frac_json::encode_frac_json(value, None, compression).unwrap())
}

fn decode_frac_json(bytes: &Vec<u8>, uses_compression: bool) -> (Value, Duration) {
    measure(|| {
        let value = frac_json::decode_frac_json(bytes, None).unwrap();
        value
    })
}

fn encode_frac_json_global_keys_table(value: &Value, compress: bool) -> (Vec<u8>, Duration) {
    let keys_table = frac_json::global_table_from_json(value);
    let mut keys_table_bytes = ByteStream::new();
    keys_table.write_keys_table(&mut keys_table_bytes).unwrap();
    let keys_table_bytes = keys_table_bytes.as_bytes();
    let compression = if compress { Some(3) } else { None };
    measure(|| frac_json::encode_frac_json(value, Some(keys_table_bytes), compression).unwrap())
}

fn find_res_folder() -> String {
    let path = std::env::current_exe().unwrap();
    let mut folder = path.parent().unwrap();
    for _ in 0..5 {
        let files = std::fs::read_dir(folder).unwrap();
        for file in files {
            let file = file.unwrap();
            let file_path = file.path().to_str().unwrap().to_string();
            let name = file.file_name();
            let name = name.to_str().unwrap();
            if name == "res" {
                return file.path().to_str().unwrap().to_string();
            }
            if name == "benchmark" {
                return Path::new(&file_path).join("res").to_str().unwrap().to_string();
            }
        }
        folder = folder.parent().unwrap();
    }
    panic!("benchmark/res/ folder not found")
}

fn get_test_files() -> Vec<(String, Value)> {
    let res_folder = find_res_folder();
    let test_files = TEST_FILE_NAMES
        .map(|file_name| Path::new(&res_folder).join(file_name).to_str().unwrap().to_string())
        .map(|file_path| {
            let bytes = if file_path.ends_with(".zst")  {
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

fn get_size_unit(size: usize) -> (f64, &'static str) {
    if size < 1024 {
        (1.0, "B")
    } else if size < 1024 * 1024 {
        (1024.0, "KB")
    } else {
        (1024.0 * 1024.0, "MB")
    }
}

fn json_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => {
            if a.len() != b.len() {
                return false;
            }
            for (key, value) in a {
                if !b.contains_key(key) {
                    return false;
                }
                if !json_eq(value, &b[key]) {
                    return false;
                }
            }
            true
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return false;
            }
            for (i, value) in a.iter().enumerate() {
                if !json_eq(value, &b[i]) {
                    return false;
                }
            }
            true
        }
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Number(a), Value::Number(b)) => {
            if a.is_f64() || b.is_f64(){
                relative_eq!(a.as_f64().unwrap(), b.as_f64().unwrap(), epsilon = 0.000001)
            } else {
                a.as_i64().unwrap() == b.as_i64().unwrap()
            }
        },
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Null, Value::Null) => true,
        _ => false,
    }
}
