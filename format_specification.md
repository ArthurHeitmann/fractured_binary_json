# Fractured binary JSON format

## Goals

Highly storage efficient across a large number of objects.

## Core idea

Reusing keys: The biggest difference between JSON and a SQL databases, is that every object value needs to have a key name. Depending on how long the name is and how much data by the value is used, the key names might make up more than half of the size. Keys that are used frequently, can be referenced using just an index. If the possible key values are known ahead of time, they can be stored separately. For remaining keys, each key is unique and only used once within a file. Repeated uses of keys, use an index referencing back to when a key was first encountered.

## File structure

### Fractured JSON file

- Header
- Root value

All text is UTF-8 encoded.

Little endian is used.

### Header

```C
struct Header {
	char[2] magic;
	uint8 config;
}
```

- `magic`: must be `FJ`
- `config`
  - `0000XXXX` version. Each new version indicates a breaking change.
  - `00010000` indicates that all bytes after the header are compressed with zstandard. This is mainly for convenience. If you really care about storage efficiency, you won't get around compression anyways, so might as well include it here.
  - `00100000` indicates that data is compressed with an external zstandard dictionary.

### Value

Only JSON data types are supported. At the root is a `value`.

```
value
	object
	array
	string
	number
	bool
	null

object
	key: value

array
	value[]
```

## Data types

The data type of a value and potentially the value itself is encoded in a single byte. The value range of 0x00 - 0xFF is mapped to different types.

0x00 - 0x0C is for primitives.

0x0D - 0x15 is for variable length data, where the length is stored in the following bytes (uint8, uint16 or uint32).

0x16 - 0xFD maps to data types where the value or length is encoded in the byte itself. Value 0 maps to the start value. The highest value maps to the end value.

The `reserved` type is reserved for potential future uses. When encountered, an error should be thrown.

| type        | start | end | count  | notes           |
|-------------|-------|-----|--------|-----------------|
| null        | 00    | 00  | 1      |                 |
| false       | 01    | 01  | 1      |                 |
| true        | 02    | 02  | 1      |                 |
| int8        | 03    | 03  | 1      |                 |
| uint8       | 04    | 04  | 1      |                 |
| int16       | 05    | 05  | 1      |                 |
| uint16      | 06    | 06  | 1      |                 |
| int32       | 07    | 07  | 1      |                 |
| uint32      | 08    | 08  | 1      |                 |
| int64       | 09    | 09  | 1      |                 |
| uint64      | 0A    | 0A  | 1      |                 |
| float       | 0B    | 0B  | 1      |                 |
| double      | 0C    | 0C  | 1      |                 |
| string 8    | 0D    | 0D  | 1      |                 |
| string 16   | 0E    | 0E  | 1      |                 |
| string 32   | 0F    | 0F  | 1      |                 |
| object 8    | 10    | 10  | 1      |                 |
| object 16   | 11    | 11  | 1      |                 |
| object 32   | 12    | 12  | 1      |                 |
| array 8     | 13    | 13  | 1      |                 |
| array 16    | 14    | 14  | 1      |                 |
| array 32    | 15    | 15  | 1      |                 |
| tiny string | 16    | 6D  | 88     | range:   0 - 87 |
| tiny object | 6E    | 9D  | 48     | range:   0 - 47 |
| tiny array  | 9E    | BD  | 32     | range:   0 - 31 |
| tiny int    | BE    | FD  | 64     | range: -32 - 31 |
| reserved    | FE    | FF  | 2      |                 |

### Element

```C
struct Element {
	uint8 dataType;
	Value value;
}
```

### Variable length data types

#### Object

```C
struct Object {
	ObjectEntry entries[]
}
```

```C
struct ObjectEntry {
	Key key;
	Element element;
}
```
`Key` starts with an `uint8` indicating how the key is encoded. The 256 possible values are mapped to the following types:

| type                    | start | end  | count |
|-------------------------|-------|------|-------|
| immediate v_uint16      | 00    | 00   | 1     |
| back reference v_uint16 | 01    | 01   | 1     |
| global index v_uint16   | 02    | 02   | 1     |
| immediate tiny_u8       | 03    | 56   | 84    |
| back reference tiny_u8  | 57    | AA   | 84    |
| global index tiny_u8    | AB    | FE   | 84    |
| reserved                | FF    | FF   | 1     |

Key values or sizes are encoded either as a variable length unsigned integer or encoded in the byte itself.

Immediate keys are encoded as strings directly after the size is encoded. Each immediate key is unique and assigned an index implicitly.  
Back reference keys reference an immediate key that was already encountered.  
Global index keys reference a key in the global keys table.  
0xFF is reserved for potential future uses. When encountered, an error should be thrown.

`tiny_u8` is encoded as `value` - `start`.

`v_uint16` is encoded as a variable length unsigned integer. One bit indicates whether another byte follows. Up to 3 bytes are allowed, to allow the full uint16 range  
`10000000` indicates that another byte follows.  
The value is encoded in the bytes b0[, b1, b2] as follows:  
`b0 & 0x7F | (b1 & 0x7F) << 7 | (b2 & 0x03) << 14`

If during decoding a key index cannot be found, an error should be thrown.

#### Array

```C
struct Array {
	Element[] element;
}
```

#### String

- UTF-8 encoded byte sequence
- has no terminator

### Numbers

When encoding, the smallest possible representation is used.

Possible integer types: tiny int, int8, int16, int32, int64, uint8, uint16, uint32, uint64.

The tiny int type maps to the values 0x16 - 0x55 in the data type byte. The value should be read
as an unsigned int and a bias of 32 subtracted from it. Giving a value range of -32 - 31.

The decision whether a floating point number is 32 bit or 64 bit encoded, is an implementation detail. Though as a guideline, if the difference is less than 0.00001%, then 32 bits can be used.

## Global Keys table

The global keys table is a list of object key names, that are shared across different files. It is stored separately and not part of the main fractured json file. Keys are referenced by index.

```C
struct GlobalKeysTable {
	uint8 config;
	uint16 count;
	KeyMapping[] keys;
}
```

`config` is a currently unused byte. Any value other than 0 should throw an error.

#### Key Mapping

```C
struct KeyMapping {
	uint16 keyLength;
	string keyName;
}
```

`keyLength`: indicates the length of the key name in bytes.
`keyName`: Name of the object key.

## Limitations

|                                      |            |
|--------------------------------------|------------|
| Unique global keys                   | 65535      |
| Longest key name                     | 65535      |
| Maximum number of object entries     | 4294967295 |
| Maximum number of array entries      | 4294967295 |
| Longest string (in bytes)            | 4294967295 |
