
import "../ByteDataWrapper.dart";
import "../DataType.dart";
import "../KeysLookup.dart";
import "../SizeClass.dart";
import "../ioClass.dart";

const _sizeClassToType = {
  SizeClass.empty: DataTypeChar.emptyString,
  SizeClass.small: DataTypeChar.smallString,
  SizeClass.big: DataTypeChar.bigString,
  SizeClass.long: DataTypeChar.longString
};
const _typeToSizeClass = {
  DataTypeChar.emptyString: SizeClass.empty,
  DataTypeChar.smallString: SizeClass.small,
  DataTypeChar.bigString: SizeClass.big,
  DataTypeChar.longString: SizeClass.long
};

class JsonString extends JsonIoClass {
  final int length;
  final String value;
  final SizeClass sizeClass;

  JsonString(this.length, this.value, this.sizeClass);

  static (DataType, JsonString?) fromJson(Object? json) {
    if (json is! String)
      throw Exception("JsonString.fromJson: json is not a String: $json");
    var string = json;
    var length = ByteDataWrapper.encodeString(string).length;
    var sizeClass = SizeClass.fromSize(length);
    var type = _sizeClassToType[sizeClass];
    if (type == null)
      throw Exception("JsonString.fromJson: invalid size class: $sizeClass");
    return (
      DataType(type),
      sizeClass != SizeClass.empty
        ? JsonString(length, string, sizeClass)
        : null
    );
  }

  static JsonString readBytes(ByteDataWrapper bytes, DataType dataType) {
    var sizeClass = _typeToSizeClass[dataType.char];
    if (sizeClass == null)
      throw Exception("JsonString.readBytes: dataType is not a string: $dataType");
    var length = sizeClass.readInt(bytes);
    var value = bytes.readString(length);
    return JsonString(length, value, sizeClass);
  }

  @override
  Object? toJson() {
    return value;
  }

  @override
  void writeBytes(ByteDataWrapper bytes, KeysLookup keysLookup) {
    sizeClass.writeInt(bytes, length);
    bytes.writeString(value);
  }

  @override
  int get size {
    return sizeClass.bytes + length;
  }
}
