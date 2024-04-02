use serde_json::Value;
use std::{
    collections::HashMap,
    io::{stdout, Write},
    path::Path,
    time::Duration,
};
use utils::Table;
use zstd::{
    bulk::{compress, decompress, Compressor, Decompressor},
    dict::from_samples,
};

use crate::utils::{get_size_unit, get_test_files, get_test_files_as_array, json_eq};

mod utils;

const TEST_FILE_NAMES_SMALL: &[&str] = &[
    "MediaContent.json.zst",
    "reddit_comments_1.json.zst",
    "reddit_posts_1.json.zst",
    "TwitterTimeline.json.zst",
    "CouchDB4k.json.zst",
];
const TEST_FILE_NAMES_BIG: &[&str] = &[
    "reddit_comments_100.json.zst",
    "reddit_posts_100.json.zst",
    "twitter.json.zst",
    "citm_catalog.json.zst",
    "canada.json.zst",
    "jeopardy.json.zst",
];
const TEST_FILE_NAMES_FOR_AVG: &[&str] = &[
    "reddit_comments_10k.json.zst",
    "reddit_posts_10k.json.zst",
    "twitter.json.zst",
    "jeopardy.json.zst",
];

// const DICT_SIZE: usize = 10 * 1024 * 1024;
const DICT_SIZE: usize = 100 * 1024;
const COMPRESSION_LEVEL: i32 = 3;

enum CompressionConfig {
    NoCompression,
    ZstdCompression,
    ZstdCompressionWithTrainedDict,
}

fn main() {
    let table_small_files = benchmark_size_relative(&TEST_FILE_NAMES_SMALL);
    let table_big_files = benchmark_size_relative(&TEST_FILE_NAMES_BIG);
    let (table_files_avg, table_encode_times, table_decode_times) = benchmark_size_relative_avg(&TEST_FILE_NAMES_FOR_AVG);
    println!(
        "\nRELATIVE SIZE RESULTS (SMALL):\n{}",
        table_small_files.to_markdown()
    );
    println!(
        "\nRELATIVE SIZE RESULTS (BIG):\n{}",
        table_big_files.to_markdown()
    );
    println!(
        "\nRELATIVE SIZE RESULTS (AVERAGE):\n{}",
        table_files_avg.to_markdown()
    );
    println!(
        "ENCODE TIME RESULTS (AVERAGE):\n{}",
        table_encode_times.to_markdown()
    );
    println!(
        "DECODE TIME RESULTS (AVERAGE):\n{}",
        table_decode_times.to_markdown()
    );
}

fn benchmark_size_relative(file_names: &[&str]) -> Table {
    let test_functions: &[(
        &str,
        fn(&Value, &String, bool, Option<&Vec<u8>>) -> (Vec<u8>, Duration),
        fn(&Vec<u8>, &String, bool, Option<&Vec<u8>>) -> (Value, Duration),
    )] = &[
        ("plain text", encode_json, decode_json),
        // ("UBJSON", encode_ubjson, decode_ubjson),
        ("MessagePack", encode_messagepack, decode_messagepack),
        ("Smile", encode_smile, decode_smile),
        ("Smile (+shared)", encode_smile_shared, decode_smile),
        ("fracture json", encode_frac_json, decode_frac_json),
        (
            "fracture json (+global table)",
            encode_frac_json_global_keys_table,
            decode_frac_json_global_keys_table,
        ),
    ];

    let test_functions: Vec<(
        &str,
        fn(&Value, &String, bool, Option<&Vec<u8>>) -> (Vec<u8>, Duration),
        fn(&Vec<u8>, &String, bool, Option<&Vec<u8>>) -> (Value, Duration),
        bool,
    )> = test_functions
        .iter()
        .map(|(name, encode, decode)| (*name, *encode, *decode, false))
        .chain(
            test_functions
                .iter()
                .map(|(name, encode, decode)| (*name, *encode, *decode, true))
                .into_iter(),
        )
        .collect();
    let test_files = get_test_files(file_names);
    let mut header: Vec<String> = vec!["".to_string()];
    let mut rows: Vec<Vec<String>> = Vec::new();

    let mut plain_text_sizes: Vec<usize> = Vec::new();
    let mut longest_log_line = 0;
    for (path, value) in test_files.iter() {
        let log_line = format!("{}", path);
        longest_log_line = longest_log_line.max(log_line.len());
        print!("\r{:<1$}", log_line, longest_log_line + 1);
        stdout().flush().unwrap();

        let size = serde_json::to_string(&value).unwrap().len();
        plain_text_sizes.push(size);
        let file_name = Path::new(&path).file_name().unwrap();
        let file_name = file_name.to_str().unwrap().to_string();
        let file_name = file_name.replace(".zst", "");
        header.push(file_name);
    }

    let mut has_tested_compression = false;
    for (i_func, (name, encode, decode, use_compression)) in test_functions.iter().enumerate() {
        if *use_compression && !has_tested_compression {
            has_tested_compression = true;
            let mut separator_row: Vec<String> = vec!["".to_string()];
            separator_row.extend(vec!["".to_string(); test_files.len()]);
            rows.push(separator_row);
        }
        let log_line = format!("{} (zstd: {:?})", name, use_compression);
        longest_log_line = longest_log_line.max(log_line.len());
        print!("\r{:<1$}", log_line, longest_log_line + 1);
        stdout().flush().unwrap();

        let mut row: Vec<String> = Vec::new();
        let name = if *use_compression {
            format!("{} (+zstd)", name)
        } else {
            name.to_string()
        };
        row.push(name);
        for (i_file, (path, value)) in test_files.iter().enumerate() {
            let (encoded, _) = encode(value, path, *use_compression, None);
            let (decoded, _) = decode(&encoded, path, *use_compression, None);
            json_eq(value, &decoded);
            let size = encoded.len();
            let plain_text_size = plain_text_sizes[i_file];
            let ratio = size as f64 / plain_text_size as f64 * 100.0;
            let mut ratio = format!("{:.1}%", ratio);
            if i_func == 0 {
                let (size_div, size_unit) = get_size_unit(plain_text_size);
                ratio = format!(
                    "{} ({:.1} {})",
                    ratio,
                    plain_text_size as f64 / size_div,
                    size_unit
                );
            }
            row.push(ratio);
        }
        rows.push(row);
    }
    println!();

    return Table { header, rows };
}

fn benchmark_size_relative_avg(file_names: &[&str]) -> (Table, Table, Table) {
    let test_function_presets: &[(
        &str,
        fn(&Value, &String, bool, Option<&Vec<u8>>) -> (Vec<u8>, Duration),
        fn(&Vec<u8>, &String, bool, Option<&Vec<u8>>) -> (Value, Duration),
    )] = &[
        ("plain text", encode_json, decode_json),
        //("UBJSON", encode_ubjson, decode_ubjson),
        ("MessagePack", encode_messagepack, decode_messagepack),
        ("Smile", encode_smile, decode_smile),
        ("Smile (+shared)", encode_smile_shared, decode_smile),
        ("fracture json", encode_frac_json, decode_frac_json),
        (
            "fracture json (+global table)",
            encode_frac_json_global_keys_table,
            decode_frac_json_global_keys_table,
        ),
    ];
    let test_functions: Vec<(_, _, _, CompressionConfig)> = test_function_presets
        .iter()
        .map(|(n, e, d)| (*n, *e, *d, CompressionConfig::NoCompression))
        .chain(
            test_function_presets
                .iter()
                .map(|(n, e, d)| (*n, *e, *d, CompressionConfig::ZstdCompression)),
        )
        .chain(test_function_presets.iter().map(|(n, e, d)| {
            (
                *n,
                *e,
                *d,
                CompressionConfig::ZstdCompressionWithTrainedDict,
            )
        }))
        .collect();
    let uses_trained_dicts = test_functions.iter().any(|(_, _, _, config)| match config {
        CompressionConfig::ZstdCompressionWithTrainedDict => true,
        _ => false,
    });

    let test_files = get_test_files_as_array(file_names);
    let mut sizes_header: Vec<String> = vec!["".to_string()];
    let mut sizes_rows: Vec<Vec<String>> = Vec::new();
    let mut encode_time_rows: Vec<Vec<String>> = Vec::new();
    let mut decode_time_rows: Vec<Vec<String>> = Vec::new();

    let mut trained_dicts: HashMap<(String, String), Vec<u8>> = HashMap::new(); // (method, file)
    let mut plain_text_sizes: Vec<f64> = Vec::new();
    let mut plain_text_encode_times: Vec<Duration> = Vec::new();
    let mut plain_text_decode_times: Vec<Duration> = Vec::new();
    let mut samples_per_row: usize = 0;
    let mut longest_log_line = 0;
    for (path, values) in test_files.iter() {
        samples_per_row += values.len();
        let mut total_size: f64 = 0.0;
        for value in values.iter() {
            let size = serde_json::to_string(&value).unwrap().len();
            total_size += size as f64;
        }
        let plain_text_size = total_size / values.len() as f64;
        plain_text_sizes.push(plain_text_size);
        let (size_div, size_unit) = get_size_unit(total_size as usize);

        let file_name = Path::new(&path).file_name().unwrap();
        let file_name = file_name.to_str().unwrap().to_string();
        let file_name = file_name.replace(".zst", "");
        let file_name = format!(
            "{} ({:.1}{} / {}{})",
            file_name,
            total_size / size_div,
            size_unit,
            values.len() / if values.len() < 1_000 { 1 } else { 1_000 },
            if values.len() < 1_000 { "" } else { "k" }
        );
        sizes_header.push(file_name);

        for (name, encode, _) in test_function_presets.iter() {
            let log_line = format!("pre process {} with {}", path, name);
            longest_log_line = longest_log_line.max(log_line.len());
            print!("\r{:<1$}", log_line, longest_log_line + 1);
            stdout().flush().unwrap();
            let dict: Vec<u8>;
            if uses_trained_dicts {
                let encoded_values: Vec<Vec<u8>> = values
                    .iter()
                    .map(|value| encode(value, path, false, None).0)
                    .collect();
                dict = from_samples(&encoded_values, DICT_SIZE).unwrap();
            } else {
                dict = Vec::new();
            }
            trained_dicts.insert((name.to_string(), path.clone()), dict);
        }
    }

    let repeat_count = 4;
    let total_samples = samples_per_row * test_functions.len() * repeat_count;
    let mut i_sample = 0;
    for (i_func, (name, encode, decode, compression_config)) in test_functions.iter().enumerate() {
        let mut sizes_row: Vec<String> = Vec::new();
        let mut encode_time_row: Vec<String> = Vec::new();
        let mut decode_time_row: Vec<String> = Vec::new();
        let row_name: String;
        match compression_config {
            CompressionConfig::NoCompression => {
                row_name = name.to_string();
            }
            CompressionConfig::ZstdCompression => {
                row_name = format!("{} (+zstd)", name);
            }
            CompressionConfig::ZstdCompressionWithTrainedDict => {
                row_name = format!("{} (+zstd +trained dict)", name);
            }
        }
        sizes_row.push(row_name.clone());
        encode_time_row.push(row_name.clone());
        decode_time_row.push(row_name);
        for (i_file, (path, values)) in test_files.iter().enumerate() {
            let file_name = Path::new(&path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let dict = trained_dicts
                .get(&(name.to_string(), path.clone()))
                .unwrap();
            let mut encoded_size_total: usize = 0;
            let mut encode_total_count: usize = 0;
            let mut encode_time_total: Duration = Duration::new(0, 0);
            let mut decode_time_total: Duration = Duration::new(0, 0);
            for _ in 0..repeat_count {
                for value in values.iter() {
                    if i_sample % 10_000 == 0 {
                        let log_line = format!(
                            "encoding with {} file {} - {:.2}%",
                            name,
                            file_name,
                            i_sample as f64 / total_samples as f64 * 100.0
                        );
                        longest_log_line = longest_log_line.max(log_line.len());
                        print!("\r{:<1$}", log_line, longest_log_line + 1);
                        stdout().flush().unwrap();
                    }
                    i_sample += 1;
                    let ((encoded, encode_duration), (decoded, decode_duration)) =
                        match *compression_config {
                            CompressionConfig::NoCompression => {
                                let encoded = encode(value, path, false, None);
                                let decoded = decode(&encoded.0, path, false, None);
                                (encoded, decoded)
                            }
                            CompressionConfig::ZstdCompression => {
                                let encoded = encode(value, path, true, None);
                                let decoded = decode(&encoded.0, path, true, None);
                                (encoded, decoded)
                            }
                            CompressionConfig::ZstdCompressionWithTrainedDict => {
                                let encoded = encode(value, path, true, Some(dict));
                                let decoded = decode(&encoded.0, path, true, Some(dict));
                                (encoded, decoded)
                            }
                        };
                    json_eq(value, &decoded);
                    encoded_size_total += encoded.len();
                    encode_total_count += 1;
                    encode_time_total += encode_duration;
                    decode_time_total += decode_duration;
                }
            }
            let size_avg = encoded_size_total as f64 / encode_total_count as f64;
            let plain_text_size = plain_text_sizes[i_file];
            let ratio = size_avg as f64 / plain_text_size as f64 * 100.0;
            let mut ratio = format!("{:.1}%", ratio);
            if i_func == 0 {
                let (size_div, size_unit) = get_size_unit(plain_text_size as usize);
                ratio = format!(
                    "{} ({:.1} {})",
                    ratio,
                    plain_text_size / size_div,
                    size_unit
                );
            }
            sizes_row.push(ratio);
            let encode_time_avg = encode_time_total / encode_total_count as u32;
            let decode_time_avg = decode_time_total / encode_total_count as u32;
            if i_func == 0 {
                plain_text_encode_times.push(encode_time_avg);
                plain_text_decode_times.push(decode_time_avg);
            }
            let encode_time_ratio = encode_time_avg.as_nanos() as f64
                / plain_text_encode_times[i_file].as_nanos() as f64;
            let decode_time_ratio = decode_time_avg.as_nanos() as f64
                / plain_text_decode_times[i_file].as_nanos() as f64;
            encode_time_row.push(format!("{:.2}x ({:.1?})", encode_time_ratio, encode_time_avg));
            decode_time_row.push(format!("{:.2}x ({:.1?})", decode_time_ratio, decode_time_avg));
        }
        sizes_rows.push(sizes_row);
        encode_time_rows.push(encode_time_row);
        decode_time_rows.push(decode_time_row);
    }
    println!();

    return (
        Table {
            header: sizes_header.clone(),
            rows: sizes_rows,
        },
        Table {
            header: sizes_header.clone(),
            rows: encode_time_rows,
        },
        Table {
            header: sizes_header,
            rows: decode_time_rows,
        },
    );
}

fn measure<T, F: Fn() -> T>(function: F) -> (T, Duration) {
    let start = std::time::Instant::now();
    let value = function();
    let elapsed = start.elapsed();
    (value, elapsed)
}

fn optionally_compress(
    data: &[u8],
    use_compression: bool,
    trained_dict: Option<&Vec<u8>>,
) -> Vec<u8> {
    if use_compression {
        match trained_dict {
            Some(trained_dict) => {
                let mut compressor =
                    Compressor::with_dictionary(COMPRESSION_LEVEL, &trained_dict).unwrap();
                compressor.compress(data).unwrap()
            }
            None => {
                let compressed = compress(data, COMPRESSION_LEVEL).unwrap();
                compressed
            }
        }
    } else {
        data.to_vec()
    }
}

fn optionally_decompress(
    data: &[u8],
    uses_compression: bool,
    trained_dict: Option<&Vec<u8>>,
) -> Vec<u8> {
    if uses_compression {
        let buffer_size = data.len() * 100;
        match trained_dict {
            Some(trained_dict) => {
                let mut decompressor = Decompressor::with_dictionary(&trained_dict).unwrap();
                decompressor.decompress(data, buffer_size).unwrap()
            }
            None => decompress(data, buffer_size).unwrap(),
        }
    } else {
        data.to_vec()
    }
}

fn encode_json(
    value: &Value,
    _: &String,
    compress: bool,
    trained_dict: Option<&Vec<u8>>,
) -> (Vec<u8>, Duration) {
    measure(|| {
        let str = serde_json::to_string(value).unwrap();
        let bytes = str.as_bytes();
        optionally_compress(bytes, compress, trained_dict)
    })
}

fn decode_json(
    bytes: &Vec<u8>,
    _: &String,
    uses_compression: bool,
    trained_dict: Option<&Vec<u8>>,
) -> (Value, Duration) {
    measure(|| {
        let bytes = optionally_decompress(bytes, uses_compression, trained_dict);
        let str = std::str::from_utf8(&bytes).unwrap();
        let value = serde_json::from_str(str).unwrap();
        value
    })
}

fn encode_smile(
    value: &impl serde::ser::Serialize,
    _: &String,
    compress: bool,
    trained_dict: Option<&Vec<u8>>,
) -> (Vec<u8>, Duration) {
    let mut serializer = serde_smile::ser::Serializer::builder();
    let serializer = serializer.shared_properties(false).shared_strings(false);
    measure(|| {
        let writer = Vec::new();
        let mut ser = serializer.build(writer);
        value.serialize(&mut ser).unwrap();
        let bytes = ser.into_inner();
        optionally_compress(&bytes, compress, trained_dict)
    })
}

fn decode_smile(
    bytes: &Vec<u8>,
    _: &String,
    uses_compression: bool,
    trained_dict: Option<&Vec<u8>>,
) -> (Value, Duration) {
    measure(|| {
        let bytes = optionally_decompress(bytes, uses_compression, trained_dict);
        serde_smile::from_slice(&bytes).unwrap()
    })
}

fn encode_smile_shared(
    value: &impl serde::ser::Serialize,
    _: &String,
    compress: bool,
    trained_dict: Option<&Vec<u8>>,
) -> (Vec<u8>, Duration) {
    let mut serializer = serde_smile::ser::Serializer::builder();
    let serializer = serializer.shared_properties(true).shared_strings(true);
    measure(|| {
        let writer = Vec::new();
        let mut ser = serializer.build(writer);
        value.serialize(&mut ser).unwrap();
        let bytes = ser.into_inner();
        optionally_compress(&bytes, compress, trained_dict)
    })
}

fn encode_messagepack(
    value: &Value,
    _: &String,
    compress: bool,
    trained_dict: Option<&Vec<u8>>,
) -> (Vec<u8>, Duration) {
    measure(|| {
        let bytes = rmp_serde::to_vec(value).unwrap();
        optionally_compress(&bytes, compress, trained_dict)
    })
}

fn decode_messagepack(
    bytes: &Vec<u8>,
    _: &String,
    uses_compression: bool,
    trained_dict: Option<&Vec<u8>>,
) -> (Value, Duration) {
    measure(|| {
        let bytes = optionally_decompress(bytes, uses_compression, trained_dict);
        rmp_serde::from_slice(&bytes).unwrap()
    })
}

// fn encode_ubjson(value: &Value, _: &String, compress: bool, trained_dict: Option<&Vec<u8>>) -> (Vec<u8>, Duration) {
//     measure(|| {
//         let bytes = serde_ub_json::to_bytes(value).unwrap();
//         optionally_compress(&bytes, compress, trained_dict)
//     })
// }

// fn decode_ubjson(bytes: &Vec<u8>, _: &String, uses_compression: bool, trained_dict: Option<&Vec<u8>>) -> (Value, Duration) {
//     measure(|| {
//         let bytes = optionally_decompress(bytes, uses_compression, trained_dict);
//         serde_ub_json::from_bytes(&bytes).unwrap()
//     })
// }

fn encode_frac_json(
    value: &Value,
    _: &String,
    compress: bool,
    trained_dict: Option<&Vec<u8>>,
) -> (Vec<u8>, Duration) {
    measure(|| {
        let bytes = frac_json::encode(value, None, None).unwrap();
        optionally_compress(&bytes, compress, trained_dict)
    })
}

fn decode_frac_json(
    bytes: &Vec<u8>,
    _: &String,
    uses_compression: bool,
    trained_dict: Option<&Vec<u8>>,
) -> (Value, Duration) {
    measure(|| {
        let bytes = optionally_decompress(bytes, uses_compression, trained_dict);
        let value = frac_json::decode(bytes, None).unwrap();
        value
    })
}

static mut CACHED_KEYS_TABLES: Option<HashMap<String, Vec<u8>>> = None;
fn get_cached_keys_tables() -> &'static mut HashMap<String, Vec<u8>> {
    unsafe {
        if CACHED_KEYS_TABLES.is_none() {
            CACHED_KEYS_TABLES = Some(HashMap::new());
        }
        CACHED_KEYS_TABLES.as_mut().unwrap()
    }
}
fn encode_frac_json_global_keys_table(
    value: &Value,
    path: &String,
    compress: bool,
    trained_dict: Option<&Vec<u8>>,
) -> (Vec<u8>, Duration) {
    let cached_keys_tables = get_cached_keys_tables();
    if !cached_keys_tables.contains_key(path) {
        let keys_table = frac_json::global_table_from_json(value).unwrap();
        cached_keys_tables.insert(path.clone(), keys_table);
    }
    let keys_table_bytes = cached_keys_tables.get(path).unwrap();
    measure(|| {
        let bytes =
            frac_json::encode(value, Some(keys_table_bytes.clone()), None).unwrap();
        optionally_compress(&bytes, compress, trained_dict)
    })
}

fn decode_frac_json_global_keys_table(
    bytes: &Vec<u8>,
    path: &String,
    uses_compression: bool,
    trained_dict: Option<&Vec<u8>>,
) -> (Value, Duration) {
    let cached_keys_tables = get_cached_keys_tables();
    let global_keys_table = cached_keys_tables.get(path).unwrap();
    measure(|| {
        let bytes = optionally_decompress(bytes, uses_compression, trained_dict);
        let value = frac_json::decode(bytes, Some(global_keys_table.clone())).unwrap();
        value
    })
}
