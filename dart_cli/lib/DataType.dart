import "ByteDataWrapper.dart";
import "SizeClass.dart";
import "ioClass.dart";

enum DataTypeChar {
  emptyObject("o"),
  smallObject("O"),
  bigObject("p"),
  longObject("P"),
  emptyArray("a"),
  smallArray("A"),
  bigArray("c"),
  longArray("C"),
  emptyString("s"),
  smallString("S"),
  bigString("t"),
  longString("T"),
  zeroInt("0"),
  int8("i"),
  uint8("I"),
  int16("j"),
  uint16("J"),
  int32("k"),
  uint32("K"),
  int64("l"),
  uint64("L"),
  zeroFloat("f"),
  float("F"),
  double("d"),
  falseBool("b"),
  trueBool("B"),
  nullValue("z");

  final String char;
  static final _charToType = {
    for (DataTypeChar type in DataTypeChar.values)
      type.char: type
  };
  static const _charsWithJsonValue = {
    DataTypeChar.emptyObject,
    DataTypeChar.emptyArray,
    DataTypeChar.emptyString,
    DataTypeChar.zeroInt,
    DataTypeChar.zeroFloat,
    DataTypeChar.falseBool,
    DataTypeChar.trueBool,
    DataTypeChar.nullValue
  };

  const DataTypeChar(this.char);

  static from(String c) {
    // TODO: compare performance
    // for (DataType dataType in DataType.values)
    //   if (dataType.char == c)
    //     return dataType;
    // throw Exception("Unknown data type char: $c");
    var dataType = _charToType[c];
    if (dataType == null)
      throw Exception("Unknown data type char: $c");
    return dataType;
  }

  SizeClass get sizeClass {
    switch (this) {
      case DataTypeChar.emptyObject:
      case DataTypeChar.emptyArray:
      case DataTypeChar.emptyString:
        return SizeClass.empty;
      case DataTypeChar.smallObject:
      case DataTypeChar.smallArray:
      case DataTypeChar.smallString:
        return SizeClass.small;
      case DataTypeChar.bigObject:
      case DataTypeChar.bigArray:
      case DataTypeChar.bigString:
        return SizeClass.big;
      case DataTypeChar.longObject:
      case DataTypeChar.longArray:
      case DataTypeChar.longString:
        return SizeClass.long;
      default:
        throw Exception("Unknown data type char: $char");
    }
  }

  bool get hasJsonValue {
    return _charsWithJsonValue.contains(this);
  }

  Object? get jsonValue {
    switch (this) {
      case DataTypeChar.emptyObject:
        return {};
      case DataTypeChar.emptyArray:
        return [];
      case DataTypeChar.emptyString:
        return "";
      case DataTypeChar.zeroInt:
        return 0;
      case DataTypeChar.zeroFloat:
        return 0.0;
      case DataTypeChar.falseBool:
        return false;
      case DataTypeChar.trueBool:
        return true;
      case DataTypeChar.nullValue:
        return null;
      default:
        throw Exception("Data type char $char has no json value");
    }
  }
}

class DataType extends IoClass {
  final DataTypeChar char;

  const DataType(this.char);

  DataType.readBytes(ByteDataWrapper bytes) : char = DataTypeChar.from(bytes.readString(1));

  @override
  void writeBytes(ByteDataWrapper bytes) {
    bytes.writeString(char.char);
  }

  @override
  int get size => 1;
}
