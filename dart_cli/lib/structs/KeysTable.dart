
import "../ByteDataWrapper.dart";
import "../ioClass.dart";
import "KeyMapping.dart";

class KeysTable extends IoClass {
  int count = 0;
  late final List<KeyMapping> mappings;
  
  KeysTable() :
    mappings = [];
  
  KeysTable.readBytes(ByteDataWrapper bytes) {
    count = bytes.readUint16();
    mappings = List.generate(count, (index) => KeyMapping.readBytes(bytes));
  }

  @override
  void writeBytes(ByteDataWrapper bytes) {
    bytes.writeUint16(count);
    for (var mapping in mappings)
      mapping.writeBytes(bytes);
  }

  @override
  int get size {
    var size = 2;
    for (var mapping in mappings)
      size += mapping.size;
    return size;
  }

  String lookupIndex(int index) {
    return mappings[index].keyName;
  }

  int? getKeyIndex(String key, {bool allowCreate = false}) {
    for (var i = 0; i < mappings.length; i++) {
      if (mappings[i].keyName == key)
        return i;
    }
    if (!allowCreate)
      return null;
    if (mappings.length >= 65536)
      throw Exception("KeysTable.getKeyIndex: too many keys");
    mappings.add(KeyMapping(mappings.length, key));
    count++;
    return mappings.length - 1;
  }

  void visitKey(String key) {
    getKeyIndex(key, allowCreate: true);
  }
}
