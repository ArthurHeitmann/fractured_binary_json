# Fractured Binary JSON

A binary JSON encoding optimized for small storage size.

For more information, see [here](https://github.com/ArthurHeitmann/fractured_binary_json.git).

## Usage

```TypeScript
import { encode, decode, keysTableFromJson } from '@raiderb/frac_json';

// basic usage
const encodedObject = encode({ key1: "value" });
const decodedObject = decode(encodedObject);

// with compression
const largeObject = { /* ... */ };
const encodedObject2 = encode(largeObject, { compressionLevel: 3 });

// with keys table
const keysTable = keysTableFromJson(largeObject); // one time only, save this to a file
// const keysTable = keysTableFromKeys(["key", "key1", "key2", "key3"]); // or generate from keys
const encodedObject3 = encode(largeObject, { globalKeysTableBytes: keysTable });
```

## Functions

```TypeScript

// Encode a JSON object (object, array, string, number, boolean, null) to a Buffer.
function encode(
	value: any,
	encodeOptions?: EncodeOptions
): Buffer
interface EncodeOptions {
	// bytes of an external keys table
	// to generate a keys table from keys, use keysTableFromKeys or keysTableFromJson
	globalKeysTableBytes?: Buffer
	// compression level for zstandard. 1-22. Default is 3.
	compressionLevel?: number
	// pre trained zstandard dictionary
	zstdDict?: Buffer
}

// Decode a Buffer to a JSON object (object, array, string, number, boolean, null).
function decode(
	fracJsonBytes: Buffer,
	decodeOptions?: DecodeOptions
): any
interface DecodeOptions {
	// bytes of an external keys table
	globalKeysTableBytes?: Buffer
	// pre trained zstandard dictionary
	zstdDict?: Buffer
}

// Generate a keys table from a list of unique keys.
// To improve performance during encoding, keys should be sorted by frequency of occurrence.
function keysTableFromKeys(
	keys: Array<string>
): Buffer

// Generate a keys table from a JSON object.
function keysTableFromJson(
	// object to recursively extract keys from
	obj: any,
	// maximum number of keys to extract
	maxCount?: number,
	// minimum number of occurrences for a key to be included
	occurrenceCutoff?: number
): Buffer
```
