# Fractured Binary JSON

A binary JSON encoding optimized for small storage size.

For more information, see [here](https://github.com/ArthurHeitmann/fractured_binary_json.git).

## Usage

```python
import frac_json as fj

# basic usage
encoded_object = fj.encode({ "key1": "value" })
decoded_object = fj.decode(encoded_object)

# with compression
large_object = {
	# ...	
}
encoded_object2 = fj.encode(large_object, compression_level=3)

# with keys table
keys_table = fj.keys_table_from_json(large_object) # one time only, save this to a file
# keys_table = keys_table_from_keys(["key", "key1", "key2", "key3"]) # or generate from keys
encoded_object3 = fj.encode(large_object, global_keys_table_bytes=keys_table)
decoded_object3 = fj.decode(encoded_object3, global_keys_table_bytes=keys_table)
```

## Functions

```python
json_type = Union[dict, list, str, int, float, bool, None]

# Encode a JSON object (object, array, string, number, boolean, null) to bytes.
def encode(
    object: json_type,
	# bytes of an external keys table
	# to generate a keys table from keys, use keys_table_from_keys or keys_table_from_json
    global_keys_table_bytes: Optional[bytes] = None,
	# compression level for zstandard. 1-22. Default is 3.
    compression_level: Optional[int] = None,
	# pre trained zstandard dictionary
    zstd_dict: Optional[bytes] = None,
) -> bytes:
    ...

# Decode bytes to a JSON object (object, array, string, number, boolean, null).
def decode(
    frac_json_bytes: bytes,
	# bytes of an external keys table
    global_keys_table_bytes: Optional[bytes] = None,
	# pre trained zstandard dictionary
    zstd_dict: Optional[bytes] = None,
) -> Any:
    ...

# Generate a keys table from a list of unique keys.
# To improve performance during encoding, keys should be sorted by frequency of occurrence.
def keys_table_from_keys(keys: List[str]) -> bytes:
    ...

# Generate a keys table from a JSON object.
def keys_table_from_json(
	# object to recursively extract keys from
    object: Any,
	# maximum number of keys to extract
    max_count: Optional[int] = None,
	# minimum number of occurrences for a key to be included
    occurrence_cutoff: Optional[int] = None,
) -> bytes:
    ...

```
