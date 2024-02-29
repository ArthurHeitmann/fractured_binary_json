# Fractured binary JSON format

## Goals

Highly storage efficient across a very large number of documents.

## Core ideas

In order to optimize storage efficiency, 3 main ideas are used:
- global keys table: The biggest difference between JSON and databases stat store data in tables, is that ever value, needs to have a key name. Depending on how long the name is and much data by the value is used, the key names along can use up close to half if not more of all storage. By replacing all keys across a large number of documents with a unique ID, a lot of repeated text can be saved. All keys used are then stored in a central table.
- binary encoding: JSON encodes everything in text form, including numbers. By storing them in binary form with varying levels of precision, a good amount of space can be saved.
- dedicated datatype for empty "objects": JSON has a small number of data types which can all be stored in a single byte. By marking empty objects (`{}`, `[]`), 0 numbers, bool and null values within the data type bytes, additional bytes for the value itself can be skipped.

## File structure

1. Header
2. Local keys table (optional)
3. Document/root element

### Header

```C
struct Header {
	char[2] magic;
	uint8 config;
}
```

- `magic`: `FJ`
- `config` is a bitmap
  - `0000XXXX` version. Each new version indicates a breaking change.
  - `00010000` indicates that a local keys table exists
  - `00100000` indicates that all bytes after the header are compressed with zstd
  - `10000000` indicates that the following byte is an additional config byte. Additional config bytes repeat as long as the last bit is `1`. These additional bytes might be used by future versions.

### Keys table

The keys table is a list of ID to key name pairs. A global table is stored separately from each file. If a key is not present in the global keys table, it is added to the local file table.

```C
struct KeysTable {
	uint16 count;
	KeyMapping[] keys;
}
```

The keys are stored in ascending order of the index.

A lookup in the global table is done directly though the index. In the local table the index is: key index - 32769.

#### Key Mapping

```C
struct KeyMapping {
	uint16 index;
	uint8 keyLength;
	char[] keyName;
}
```

`index`: Global key indices go from 0 to 2¹⁵ (32768). Local key indices start at 2¹⁵ + 1 (32769).
`key length`: indicates the length of the key name in bytes.
`key name`: Name of the key.

### Document

The high level data types are the same as with JSON. At the top level is an `element`.

```
element
	value

value
	object
	array
	string
	number
	bool
	null

object
	key: element

array
	element[]
```

## Data types

The data type is denoted by a single char. It specifies the JSON data type as well as the size class.

All text is UTF-8 encoded.

Little endian is used.

Data types:
- object
	- empty object `{}` (o)
	- small object (O)
	- big object (p)
	- long object (P)
- array
	- empty array `[]` (a)
	- small array (A)
	- big array (c)
	- long array (C)
- string
	- empty string `""` (s)
	- small string (S)
	- big string (t)
	- long string (T)
- number
	- integer
		- 0 (0)
		- int8 (i)
		- uint8 (I)
		- int16 (j)
		- uint16 (J)
		- int32 (k)
		- uint32 (K)
		- int64 (l)
		- uint64 (L)
	- floating point
		- 0.0 (f)
		- float (F)
		- double (d)
- bool
	- false (b)
	- true (B)
- null (z)

#### Data sizes

The data type for objects, arrays, and strings specifies 4 size classes:

| class | size |            |
|-------|------|------------|
| empty | 0    |            |
| small | 2⁸   | 256        |
| big   | 2¹⁶  | 65536      |
| long  | 2³²  | 4294967296 |

The `size` data type in the following structs is based on the size class specified in the data type.

#### Element

```C
struct Element {
	char dataType;
	Value value;
}
```

#### Object

```C
struct Object {
	size count;
	ObjectEntry entries[]
}
```

```C
struct ObjectEntry {
	uint16 keyIndex;
	Element element;
}
```

#### Array

```C
struct Array {
	size count;
	Element[] element;
}
```

#### String

```C
struct String {
	size count;
	char[] string;
}
```

- UTF-8 encoded
- no terminator

#### Numbers

When encoding the smallest possible representation is used.

Possible integer types: int8, int16, int32, int64, uint8, uint16, uint32, uint64.  
For now floating point numbers are all float.

## Limitations

|                                      |            |
|--------------------------------------|------------|
| Total possible number of unique keys | 65536      |
| Unique global keys                   | 32768      |
| Unique local keys                    | 32768      |
| Longest key name                     | 256        |
| Maximum number of object entries     | 4294967296 |
| Maximum number of array entries      | 4294967296 |
| Longest string (in bytes)            | 4294967296 |
