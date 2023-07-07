
import "dart:convert";
import "dart:io";

import "ByteDataWrapper.dart";
import "structs/KeysTable.dart";

Future<void> generateGlobalKeysTableFromFiles(List<String> jsonFiles, String globalKeysTablePath) async {
  var allJsonObjects = <Object>[];
  for (var jsonFile in jsonFiles) {
    var jsonStr = await File(jsonFile).readAsString();
    var json = jsonDecode(jsonStr);
    allJsonObjects.add(json);
  }
  await generateGlobalKeysTableFromJson(allJsonObjects, globalKeysTablePath);
}

Future<void> generateGlobalKeysTableFromJson(List<Object> objects, String globalKeysTablePath) async {
  KeysTable keysTable;
  if (await File(globalKeysTablePath).exists()) {
    var keysTableBytes = await ByteDataWrapper.fromFile(globalKeysTablePath);
    keysTable = KeysTable.readBytes(keysTableBytes);
  }
  else {
    keysTable = KeysTable();
  }
  for (var object in objects)
    _visitJson(object, keysTable);
  
  var keysTableBytes = ByteDataWrapper.allocate(keysTable.size);
  keysTable.writeBytes(keysTableBytes);
  await keysTableBytes.toFile(globalKeysTablePath);
}

void _visitJson(Object? json, KeysTable keysTable) {
  if (json is Map) {
    for (var key in json.keys)
      keysTable.visitKey(key);
    for (var value in json.values)
      _visitJson(value, keysTable);
  }
  if (json is List) {
    for (var item in json)
      _visitJson(item, keysTable);
  }
}
