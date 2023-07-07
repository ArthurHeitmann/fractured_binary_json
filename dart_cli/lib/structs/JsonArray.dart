
import "../ByteDataWrapper.dart";
import "../DataType.dart";
import "../KeysLookup.dart";
import "../SizeClass.dart";
import "../ioClass.dart";
import "Element.dart";

const _sizeClassToType = {
  SizeClass.empty: DataTypeChar.emptyArray,
  SizeClass.small: DataTypeChar.smallArray,
  SizeClass.big: DataTypeChar.bigArray,
  SizeClass.long: DataTypeChar.longArray
};
const _typeToSizeClass = {
  DataTypeChar.emptyArray: SizeClass.empty,
  DataTypeChar.smallArray: SizeClass.small,
  DataTypeChar.bigArray: SizeClass.big,
  DataTypeChar.longArray: SizeClass.long
};

class JsonArray extends JsonIoClass {
  final int length;
  final List<Element> values;
  final SizeClass sizeClass;

  JsonArray(this.length, this.values, this.sizeClass);

  static (DataType, JsonArray?) fromJson(Object? json, KeysLookup keysLookup) {
    if (json is! List)
      throw Exception("JsonArray.fromJson: json is not a List: $json");
    var array = json;
    var length = array.length;
    var sizeClass = SizeClass.fromSize(length);
    var type = _sizeClassToType[sizeClass];
    if (type == null)
      throw Exception("JsonArray.fromJson: invalid size class: $sizeClass");
    var arrayValues = array.map((e) => Element.fromJson(e, keysLookup)).toList();
    return (
      DataType(type),
      sizeClass != SizeClass.empty
        ? JsonArray(length, arrayValues, sizeClass)
        : null
    );
  }

  static JsonArray readBytes(ByteDataWrapper bytes, DataType dataType, KeysLookup keysLookup) {
    var sizeClass = _typeToSizeClass[dataType.char];
    if (sizeClass == null)
      throw Exception("JsonArray.readBytes: dataType is not a array: $dataType");
    var length = sizeClass.readInt(bytes);
    var arrayValues = List.generate(length, (i) => Element.readBytes(bytes, keysLookup));
    return JsonArray(length, arrayValues, sizeClass);
  }

  @override
  Object? toJson() {
    return values.map((e) => e.toJson()).toList();
  }

  @override
  void writeBytes(ByteDataWrapper bytes, KeysLookup keysLookup) {
    sizeClass.writeInt(bytes, length);
    for (var value in values)
      value.writeBytes(bytes, keysLookup);
  }
  
  @override
  int get size {
    var size = sizeClass.bytes;
    for (var value in values)
      size += value.size;
    return size;
  }
}
