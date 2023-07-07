
import "../ByteDataWrapper.dart";
import "../DataType.dart";
import "../KeysLookup.dart";
import "../SizeClass.dart";
import "../ioClass.dart";
import "Element.dart";

const _sizeClassToType = {
  SizeClass.empty: DataTypeChar.emptyObject,
  SizeClass.small: DataTypeChar.smallObject,
  SizeClass.big: DataTypeChar.bigObject,
  SizeClass.long: DataTypeChar.longObject
};
const _typeToSizeClass = {
  DataTypeChar.emptyObject: SizeClass.empty,
  DataTypeChar.smallObject: SizeClass.small,
  DataTypeChar.bigObject: SizeClass.big,
  DataTypeChar.longObject: SizeClass.long
};

class JsonObject extends JsonIoClass {
  final int length;
  final List<JsonObjectEntry> entries;
  final SizeClass sizeClass;

  JsonObject(this.length, this.entries, this.sizeClass);

  static (DataType, JsonObject?) fromJson(Object? json, KeysLookup keysLookup) {
    if (json is! Map)
      throw Exception("JsonObject.fromJson: json is not a Map: $json");
    var object = json;
    var length = object.length;
    var sizeClass = SizeClass.fromSize(length);
    var type = _sizeClassToType[sizeClass];
    if (type == null)
      throw Exception("JsonObject.fromJson: invalid size class: $sizeClass");
    var objectEntries = object.entries
      .map((e) => JsonObjectEntry.fromMapEntry(e as MapEntry<String, Object?>, keysLookup))
      .toList();
    return (
      DataType(type),
      sizeClass != SizeClass.empty
        ? JsonObject(length, objectEntries, sizeClass)
        : null
    );
  }

  static JsonObject readBytes(ByteDataWrapper bytes, DataType dataType, KeysLookup keysLookup) {
    var sizeClass = _typeToSizeClass[dataType.char];
    if (sizeClass == null)
      throw Exception("JsonObject.readBytes: dataType is not a object: $dataType");
    var length = sizeClass.readInt(bytes);
    var objectEntries = List.generate(
      length,
      (i) => JsonObjectEntry.readBytes(bytes, keysLookup)
    );
    return JsonObject(length, objectEntries, sizeClass);
  }

  @override
  Object? toJson() {
    return Map.fromEntries(entries.map((e) => e.toJson()));
  }

  @override
  void writeBytes(ByteDataWrapper bytes, KeysLookup keysLookup) {
    sizeClass.writeInt(bytes, length);
    for (var entry in entries)
      entry.writeBytes(bytes, keysLookup);
  }

  @override
  int get size {
    var size = sizeClass.bytes;
    for (var entry in entries)
      size += entry.size;
    return size;
  }
}

class JsonObjectEntry extends JsonIoClass {
  final String key;
  final Element value;

  JsonObjectEntry(this.key, this.value);

  static JsonObjectEntry fromMapEntry(MapEntry<String, Object?> entry, KeysLookup keysLookup) {
    var key = entry.key;
    var value = Element.fromJson(entry.value, keysLookup);
    keysLookup.visitKey(key);
    return JsonObjectEntry(key, value);
  }

  static JsonObjectEntry readBytes(ByteDataWrapper bytes, KeysLookup keysLookup) {
    var key = keysLookup.lookupIndex(bytes.readUint16());
    var value = Element.readBytes(bytes, keysLookup);
    return JsonObjectEntry(key, value);
  }

  @override
  MapEntry toJson() {
    return MapEntry(key, value.toJson());
  }

  @override
  void writeBytes(ByteDataWrapper bytes, KeysLookup keysLookup) {
    bytes.writeUint16(keysLookup.getKeyIndex(key));
    value.writeBytes(bytes, keysLookup);
  }

  @override
  int get size {
    return 2 + value.size;
  }
}
