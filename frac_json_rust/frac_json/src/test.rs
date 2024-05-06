
#[cfg(test)]
mod tests {
	use serde_json::Value;
	use crate::{encode, decode};


	fn test_file(s: &str, encoded_size: usize) {
		let value = serde_json::from_str::<Value>(s).unwrap();
		let frac_json_bytes = encode(&value, None, None, None).unwrap();
		assert_eq!(encoded_size, frac_json_bytes.len());
		let decoded_value = decode(&frac_json_bytes, None, None).unwrap();
		assert_eq!(value, decoded_value);
	}

	fn test_file_compressed(s: &str) {
		let value = serde_json::from_str::<Value>(s).unwrap();
		let frac_json_bytes = encode(&value, None, Some(3), None).unwrap();
		let decoded_value = decode(&frac_json_bytes, None, None).unwrap();
		assert_eq!(value, decoded_value);
	}

	#[test]
	fn test_null() {
		let s = include_str!("./test_files/null.json");
		test_file(s, 4);
	}

	#[test]
	fn test_false() {
		let s = include_str!("./test_files/bool_false.json");
		test_file(s, 4);
	}

	#[test]
	fn test_true() {
		let s = include_str!("./test_files/bool_true.json");
		test_file(s, 4);
	}

	#[test]
	fn test_int8() {
		let s = include_str!("./test_files/int8.json");
		test_file(s, 5);
	}

	#[test]
	fn test_uint8() {
		let s = include_str!("./test_files/uint8.json");
		test_file(s, 5);
	}

	#[test]
	fn test_int16() {
		let s = include_str!("./test_files/int16.json");
		test_file(s, 6);
	}

	#[test]
	fn test_uint16() {
		let s = include_str!("./test_files/uint16.json");
		test_file(s, 6);
	}

	#[test]
	fn test_int32() {
		let s = include_str!("./test_files/int32.json");
		test_file(s, 8);
	}

	#[test]
	fn test_uint32() {
		let s = include_str!("./test_files/uint32.json");
		test_file(s, 8);
	}

	#[test]
	fn test_int64() {
		let s = include_str!("./test_files/int64.json");
		test_file(s, 12);
	}

	#[test]
	fn test_uint64() {
		let s = include_str!("./test_files/uint64.json");
		test_file(s, 12);
	}

	#[test]
	fn test_float() {
		let s = include_str!("./test_files/float.json");
		test_file(s, 8);
	}

	#[test]
	fn test_double() {
		let s = include_str!("./test_files/double.json");
		test_file(s, 12);
	}

	#[test]
	fn test_string8() {
		let s = include_str!("./test_files/string8.json");
		test_file(s, 3+1+1+100);
	}

	#[test]
	fn test_string16() {
		let s = include_str!("./test_files/string16.json");
		test_file(s, 3+1+2+500);
	}

	#[test]
	fn test_string16_compressed() {
		let s = include_str!("./test_files/string16.json");
		test_file_compressed(s);
	}

	#[test]
	fn test_object8() {
		let s = include_str!("./test_files/object8.json");
		test_file(s, 3+1+1+100*2+1082);
	}

	#[test]
	fn test_object16() {
		let s = include_str!("./test_files/object16.json");
		test_file(s, 3+1+2+300*2+3189);
	}

	#[test]
	fn test_array8() {
		let s = include_str!("./test_files/array8.json");
		test_file(s, 3+1+1+32+(49-31)*2);
	}

	#[test]
	fn test_array16() {
		let s = include_str!("./test_files/array16.json");
		test_file(s, 3+1+2+32+(256-31)*2+(300-256)*3);
	}

	#[test]
	fn test_tiny_string() {
		let s = include_str!("./test_files/tiny_string.json");
		test_file(s, 3+1+13);
	}

	#[test]
	fn test_empty_string() {
		let s = include_str!("./test_files/empty_string.json");
		test_file(s, 4);
	}

	#[test]
	fn test_tiny_object() {
		let s = include_str!("./test_files/tiny_object.json");
		test_file(s, 3+1+3);
	}

	#[test]
	fn test_empty_object() {
		let s = include_str!("./test_files/empty_object.json");
		test_file(s, 4);
	}

	#[test]
	fn test_tiny_array() {
		let s = include_str!("./test_files/tiny_array.json");
		test_file(s, 7);
	}

	#[test]
	fn test_empty_array() {
		let s = include_str!("./test_files/empty_array.json");
		test_file(s, 4);
	}

	#[test]
	fn test_tiny_int() {
		let s = include_str!("./test_files/tiny_int.json");
		test_file(s, 4);
	}
	
	#[test]
	fn test_combined() {
		let s = include_str!("./test_files/combined.json");
		test_file(s, 3+1+18*2+123+2*1+2*2+2*4+2*8+4+8+1+94+3*2+27+4+21+13);
	}
	
	#[test]
	fn test_combined_compressed() {
		let s = include_str!("./test_files/combined.json");
		test_file_compressed(s);
	}
}
