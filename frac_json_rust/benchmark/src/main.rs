use std::{io::{stdout, Write}, path::Path};

use frac_json::ByteStream;
use serde_json::Value;
use zstd::bulk::compress;


fn main() {
    let test_functions: Vec<(&str, fn(&Value, bool) -> Vec<u8>)> = vec![
        ("plain text", encoded_json),
        ("Smile", encode_smile),
        ("Smile (+ shared properties)", encode_smile_shared_properties),
        ("MessagePack", encode_messagepack),
        // ("CBOR", encode_cbor),
        // ("UBJSON", encode_ubjson),
        ("fracture json", encode_frac_json),
        ("fracture json (+global table)", encode_frac_json_global_keys_table),
    ];
    let test_functions: Vec<(&str, fn(&Value, bool) -> Vec<u8>, bool)> = test_functions
        .iter()
        .map(|(name, function)| (*name, *function, false))
        .chain(
            test_functions
                .iter()
                .map(|(name, function)| (*name, *function, true))
                .into_iter(),
        )
        .collect();
    let test_files = get_test_files();
    let mut results: Vec<(String, Vec<(usize, f64, f64)>)> = Vec::new();
    
    let mut plain_text_sizes: Vec<usize> = Vec::new();
    for (_, value) in test_files.iter() {
        let size = serde_json::to_string(&value).unwrap().len();
        plain_text_sizes.push(size);
    }

    for (name, function, use_compression) in test_functions.iter() {
        print!("\r{} (zstd: {:?})", name, use_compression);
        stdout().flush().unwrap();
        let mut relative_sizes: Vec<(usize, f64, f64)> = Vec::new();
        for (i_file, (_, value)) in test_files.iter().enumerate() {
            let start = std::time::Instant::now();
            let encoded = function(&value, *use_compression);
            let elapsed = start.elapsed();
            let size = encoded.len();
            let plain_text_size = plain_text_sizes[i_file];
            let ratio = size as f64 / plain_text_size as f64;
            let time = elapsed.as_nanos() as f64 / 1000.0;
            relative_sizes.push((size, ratio, time));
        }
        results.push((name.to_string(), relative_sizes));
    }
    println!();

    
    // let mut file_names: Vec<String> = Vec::new();
    // for file_path in test_files.iter() {
    //     let file_name = Path::new(&file_path).file_name().unwrap();
    //     file_names.push(file_name.to_str().unwrap().to_string());
    // }
    // let max_name_length = results.iter().map(|s| s.0.len()).max().unwrap() + 2;
    // print!("Format{}", " ".repeat(max_name_length - "Format".len()));
    // for file_name in file_names.iter() {
    //     print!("{} ", file_name);
    // }
    // println!();
    // for (name, relative_sizes) in results {
    //     print!("{}{}", name, " ".repeat(max_name_length - name.len()));
    //     for size in relative_sizes {
    //         print!("{:.2}%\t", size.1 * 100.0);
    //         // print!("{}\t", size.0);
    //     }
    //     println!();
    // }

    let mut column_units: Vec<(f64, &str)> = Vec::new();
    let mut table_rel: Vec<Vec<String>> = Vec::new();
    let mut table_abs: Vec<Vec<String>> = Vec::new();
    let mut table_time: Vec<Vec<String>> = Vec::new();
    let mut header: Vec<String> = Vec::new();
    header.push("Format".to_string());
    header.push("zstd".to_string());
    for (i_file, (file_path, _)) in test_files.iter().enumerate() {
        let column_sizes = results
            .iter()
            .map(|r| r.1[i_file].0)
            .collect::<Vec<usize>>();
        column_units.push(get_sizes_unit(column_sizes));
        let file_name = Path::new(&file_path).file_name().unwrap();
        let file_name = file_name.to_str().unwrap().to_string();
        header.push(format!("{} [{}]", file_name, column_units[i_file].1));
    }
    table_rel.push(header.clone());
    table_abs.push(header.clone());
    table_time.push(header.clone());
    for (i_res, (name, format_results)) in results.iter().enumerate() {
        let mut row_rel: Vec<String> = Vec::new();
        let mut row_abs: Vec<String> = Vec::new();
        let mut row_time: Vec<String> = Vec::new();
        row_rel.push(name.to_string());
        row_abs.push(name.to_string());
        row_time.push(name.to_string());
        row_rel.push(if test_functions[i_res].2 { "y" } else { "n" }.to_string());
        row_abs.push(if test_functions[i_res].2 { "y" } else { "n" }.to_string());
        row_time.push(if test_functions[i_res].2 { "y" } else { "n" }.to_string());
        for (i_file, size) in format_results.iter().enumerate() {
            row_rel.push(format!("{:.3}", size.1));
            let (size_div, unit) = column_units[i_file];
            if unit == "B" {
                row_abs.push(format!("{}", size.0));
            } else {
                row_abs.push(format!("{:.2}", size.0 as f64 / size_div));
            }
            row_time.push(format!("{:.2}", size.2));
        }
        table_rel.push(row_rel);
        table_abs.push(row_abs);
        table_time.push(row_time);
    }

    for row in table_rel {
        println!("{}", row.join(","));
    }
    println!();
    for row in table_abs {
        println!("{}", row.join(","));
    }
    println!();
    for row in table_time {
        println!("{}", row.join(","));
    }

}

fn optionally_compress(data: &[u8], use_compression: bool) -> Vec<u8> {
    if use_compression {
        let compressed = compress(data, 3).unwrap();
        compressed
    } else {
        data.to_vec()
    }
}

fn encoded_json(value: &Value, compress: bool) -> Vec<u8> {
    let str = serde_json::to_string(value).unwrap();
    let bytes = str.as_bytes();
    optionally_compress(bytes, compress)
}

fn encode_smile(value: &Value, compress: bool) -> Vec<u8> {
    let bytes = serde_smile::to_vec(value).unwrap();
    optionally_compress(&bytes, compress)
}

fn encode_smile_shared_properties(value: &impl serde::ser::Serialize, compress: bool) -> Vec<u8> {
    let mut serializer = serde_smile::ser::Serializer::builder();
    let serializer = serializer
        .shared_properties(true)
        .shared_strings(true);
    let writer = Vec::new();
    let mut ser = serializer.build(writer);
    value.serialize(&mut ser).unwrap();
    let bytes = ser.into_inner();
    optionally_compress(&bytes, compress)
}

fn encode_messagepack(value: &Value, compress: bool) -> Vec<u8> {
    let bytes = rmp_serde::to_vec(value).unwrap();
    optionally_compress(&bytes, compress)
}

// fn encode_cbor(value: &Value, compress: bool) -> Vec<u8> {
//     let bytes = serde_cbor::to_vec(value).unwrap();
//     optionally_compress(&bytes, compress)
// }

// fn encode_ubjson(value: &Value, compress: bool) -> Vec<u8> {
//     let bytes = serde_ub_json::to_bytes(value).unwrap();
//     optionally_compress(&bytes, compress)
// }

fn encode_frac_json(value: &Value, compress: bool) -> Vec<u8> {
    let compression = if compress { Some(3) } else { None };
    frac_json::encode_frac_json(value, None, compression).unwrap()
}

fn encode_frac_json_global_keys_table(value: &Value, compress: bool) -> Vec<u8> {
    let keys_table = frac_json::global_table_from_json(value);
    let mut keys_table_bytes = ByteStream::new();
    keys_table.write_keys_table(&mut keys_table_bytes).unwrap();
    let keys_table_bytes = keys_table_bytes.as_bytes();
    let compression = if compress { Some(3) } else { None };
    frac_json::encode_frac_json(value, Some(keys_table_bytes), compression).unwrap()
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
    panic!("res folder not found")
}

fn get_test_files() -> Vec<(String, Value)> {
    let res_folder = find_res_folder();
    println!("res_folder: {}", res_folder);
    let entries = std::fs::read_dir(res_folder).unwrap();
    entries
        .map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            path.to_str().unwrap().to_string()
        })
        .filter(|s| s.ends_with(".json.zst"))
        .map(|file_path| {
            let compressed_bytes_reader = std::fs::File::open(&file_path).unwrap();
            let decompressed_bytes = zstd::decode_all(compressed_bytes_reader).unwrap();
            let decompressed = String::from_utf8(decompressed_bytes).unwrap();
            let value = serde_json::from_str(&decompressed).unwrap();
            (file_path.to_string(), value)
        })
        .collect()
}

fn get_sizes_unit(row: Vec<usize>) -> (f64, &'static str) {
    let min = *row.iter().min().unwrap();
    if min < 1024 {
        (1.0, "B")
    } else if min < 1024 * 1024 {
        (1024.0, "KB")
    } else {
        (1024.0 * 1024.0, "MB")
    }
}
