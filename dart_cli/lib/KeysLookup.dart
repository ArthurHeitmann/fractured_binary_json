
import "structs/KeysTable.dart";

const _globalTableEnd = 0x8000;
const _localTableOffset = 0x8001;

class KeysLookup {
  final KeysTable globalKeysTable;
  KeysTable? localKeysTable;

  KeysLookup(this.globalKeysTable, [this.localKeysTable]);

  String lookupIndex(int index) {
    if (index <= _globalTableEnd) {
      return globalKeysTable.lookupIndex(index);
    } else {
      if (localKeysTable == null)
        throw Exception("Local keys table not found");
      return localKeysTable!.lookupIndex(index - _localTableOffset);
    }
  }

  int getKeyIndex(String key) {
    var index = globalKeysTable.getKeyIndex(key);
    if (index != null)
      return index;
    localKeysTable ??= KeysTable();
    index = localKeysTable!.getKeyIndex(key, allowCreate: true);
    if (index == null)
      throw Exception("Could not create key in local keys table");
    return index + _localTableOffset;
  }

  void visitKey(String key) {
    getKeyIndex(key);
  }
}
