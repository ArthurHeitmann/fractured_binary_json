
import "../ByteDataWrapper.dart";
import "../DataType.dart";
import "../KeysLookup.dart";
import "../ioClass.dart";
import "JsonArray.dart";
import "JsonFloat.dart";
import "JsonInt.dart";
import "JsonObject.dart";
import "JsonString.dart";

class Element extends JsonIoClass {
  late final DataType dataType;
  late final JsonIoClass? value;

  Element(this.dataType, this.value);

  Element.fromJson(Object? json, KeysLookup keysLookup) {
    (DataType, JsonIoClass?) result;
    if (json is int)
      result = JsonInt.fromJson(json);
    else if (json is double)
      result = JsonFloat.fromJson(json);
    else if (json is bool)
      result = (
        DataType(json ? DataTypeChar.trueBool : DataTypeChar.falseBool),
        null
      );
    else if (json == null)
      result = (DataType(DataTypeChar.nullValue), null);
    else if (json is List)
      result = JsonArray.fromJson(json, keysLookup);
    else if (json is Map)
      result = JsonObject.fromJson(json, keysLookup);
    else if (json is String)
      result = JsonString.fromJson(json);
    else
      throw Exception("Element.fromJson: json is not a valid type: $json");

    dataType = result.$1;
    value = result.$2;
  }

  Element.readBytes(ByteDataWrapper bytes, KeysLookup keysLookup) {
    dataType = DataType.readBytes(bytes);
    if (!dataType.char.hasJsonValue)
      value = _parseValue(dataType, bytes, keysLookup);
  }

  JsonIoClass _parseValue(DataType dataType, ByteDataWrapper bytes, KeysLookup keysLookup) {
    switch (dataType.char) {
      case DataTypeChar.smallObject:
      case DataTypeChar.bigObject:
      case DataTypeChar.longObject:
        return JsonObject.readBytes(bytes, dataType, keysLookup);
      case DataTypeChar.smallArray:
      case DataTypeChar.bigArray:
      case DataTypeChar.longArray:
        return JsonArray.readBytes(bytes, dataType, keysLookup);
      case DataTypeChar.smallString:
      case DataTypeChar.bigString:
      case DataTypeChar.longString:
        return JsonString.readBytes(bytes, dataType);
      case DataTypeChar.int8:
      case DataTypeChar.uint8:
      case DataTypeChar.int16:
      case DataTypeChar.uint16:
      case DataTypeChar.int32:
      case DataTypeChar.uint32:
      case DataTypeChar.int64:
      case DataTypeChar.uint64:
        return JsonInt.readBytes(bytes, dataType);
      case DataTypeChar.float:
      case DataTypeChar.double:
        return JsonFloat.readBytes(bytes, dataType);
      default:
        throw Exception("Element._parseValue: unexpected data type: ${dataType.char}");
    }
  }

  @override
  Object? toJson() {
    return dataType.char.hasJsonValue
      ? dataType.char.jsonValue
      : value!.toJson();
  }

  @override
  void writeBytes(ByteDataWrapper bytes, KeysLookup keysLookup) {
    dataType.writeBytes(bytes);
    value?.writeBytes(bytes, keysLookup);    
  }
  
  @override
  int get size {
    var size = dataType.size;
    if (value != null)
      size += value!.size;
    return size;
  }
}
