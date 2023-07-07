
import "../../ByteDataWrapper.dart";
import "../../DataType.dart";
import "../../KeysLookup.dart";
import "../../ioClass.dart";

typedef _IntRange = (int, int);
const _IntRange _int8Range = (-128, 127);
const _IntRange _uint8Range = (0, 255);
const _IntRange _int16Range = (-32768, 32767);
const _IntRange _uint16Range = (0, 65535);
const _IntRange _int32Range = (-2147483648, 2147483647);
const _IntRange _uint32Range = (0, 4294967295);
const _IntRange _int64Range = (-9223372036854775808, 9223372036854775807);

typedef IntReadWrite = (int Function(ByteDataWrapper), void Function(ByteDataWrapper, int), int);
final IntReadWrite _int8RW = (
  (bytes) => bytes.readInt8(),
  (bytes, value) => bytes.writeInt8(value),
  8
);
final IntReadWrite _uint8RW = (
  (bytes) => bytes.readUint8(),
  (bytes, value) => bytes.writeUint8(value),
  8
);
final IntReadWrite _int16RW = (
  (bytes) => bytes.readInt16(),
  (bytes, value) => bytes.writeInt16(value),
  16
);
final IntReadWrite _uint16RW = (
  (bytes) => bytes.readUint16(),
  (bytes, value) => bytes.writeUint16(value),
  16
);
final IntReadWrite _int32RW = (
  (bytes) => bytes.readInt32(),
  (bytes, value) => bytes.writeInt32(value),
  32
);
final IntReadWrite _uint32RW = (
  (bytes) => bytes.readUint32(),
  (bytes, value) => bytes.writeUint32(value),
  32
);
final IntReadWrite _int64RW = (
  (bytes) => bytes.readInt64(),
  (bytes, value) => bytes.writeInt64(value),
  32
);
final IntReadWrite _uint64RW = (
  (bytes) => bytes.readUint64(),
  (bytes, value) => bytes.writeUint64(value),
  32
);
final Map<DataTypeChar, IntReadWrite> _dataTypeToRW = {
  DataTypeChar.int8: _int8RW,
  DataTypeChar.uint8: _uint8RW,
  DataTypeChar.int16: _int16RW,
  DataTypeChar.uint16: _uint16RW,
  DataTypeChar.int32: _int32RW,
  DataTypeChar.uint32: _uint32RW,
  DataTypeChar.int64: _int64RW,
  DataTypeChar.uint64: _uint64RW
};

class JsonInt extends JsonIoClass {
  final int value;
  final IntReadWrite readWrite;
  final int _size;

  const JsonInt(this.value, this.readWrite, this._size);

  static (DataType, JsonInt?) fromJson(Object? json) {
    if (json is! int)
      throw Exception("JsonInt.fromJson: json is not an int: $json");
    var number = json;
    (DataTypeChar, IntReadWrite?) result;
    if (number == 0)
      result = (DataTypeChar.zeroInt, null);
    else if (number < 0) {
      if (number >= _int8Range.$1)
        result = (DataTypeChar.int8, _int8RW);
      else if (number >= _int16Range.$1)
        result = (DataTypeChar.int16, _int16RW);
      else if (number >= _int32Range.$1)
        result = (DataTypeChar.int32, _int32RW);
      else if (number >= _int64Range.$1)
        result = (DataTypeChar.int64, _int64RW);
      else
        throw Exception("JsonInt.fromJson: number is too small: $number");
    } else {
      if (number <= _uint8Range.$2)
        result = (DataTypeChar.uint8, _uint8RW);
      else if (number <= _uint16Range.$2)
        result = (DataTypeChar.uint16, _uint16RW);
      else if (number <= _uint32Range.$2)
        result = (DataTypeChar.uint32, _uint32RW);
      else
        result = (DataTypeChar.uint64, _uint64RW);
    }

    return (
      DataType(result.$1),
      result.$2 == null ? null : JsonInt(number, result.$2!, result.$2!.$3)
    );
  }

  static JsonInt readBytes(ByteDataWrapper bytes, DataType dataType) {
    var intRW = _dataTypeToRW[dataType.char];
    if (intRW == null)
      throw Exception("JsonInt.readBytes: dataType is not an int: $dataType");
    int value = intRW.$1(bytes);
    return JsonInt(value, intRW, intRW.$3);
  }

  @override
  Object? toJson() {
    return value;
  }

  @override
  void writeBytes(ByteDataWrapper bytes, KeysLookup keysLookup) {
    readWrite.$2(bytes, value);
  }

  @override
  int get size => _size;
}
