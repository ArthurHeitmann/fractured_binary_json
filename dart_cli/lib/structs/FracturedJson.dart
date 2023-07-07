
import "dart:typed_data";

import "package:es_compression/zstd.dart";

import "../ByteDataWrapper.dart";
import "../KeysLookup.dart";
import "../ioClass.dart";
import "Element.dart";
import "Header.dart";
import "KeysTable.dart";

class FracturedJson extends JsonIoClass {
  late final Header header;
  late final Element rootElement;
  late final KeysLookup keysLookup;
  
  FracturedJson.fromJson(Object? json,  KeysTable globalKeysTable) {
    keysLookup = KeysLookup(globalKeysTable);
    header = Header();
    rootElement = Element.fromJson(json, keysLookup);
    updateSize();
  }
  
  FracturedJson.readBytes(ByteDataWrapper bytes, KeysTable globalKeysTable) {
    header = Header.readBytes(bytes);
    if (header.useZstd) {
      var decompressed = zstd.decode(bytes.asUint8List(bytes.length - bytes.position));
      var buffer = ByteData(decompressed.length).buffer;
      bytes = ByteDataWrapper(buffer);
    }
    KeysTable? localKeysTable;
    if (header.hasLocalKeysTable)
      localKeysTable = KeysTable.readBytes(bytes);
    else
      localKeysTable = null;
    keysLookup = KeysLookup(globalKeysTable, localKeysTable);
    rootElement = Element.readBytes(bytes, keysLookup);
  }

  @override
  Object? toJson() {
    return rootElement.toJson();
  }

  @override
  void writeBytes(ByteDataWrapper bytes, [KeysLookup? keysLookup]) {
    keysLookup ??= this.keysLookup;
    updateSize();
    header.writeBytes(bytes);
    if (header.useZstd) {
      var compressedBytes = ByteDataWrapper.allocate((bytes.length / 2 * 1.2).ceil(), autoGrow: true);
      if (header.hasLocalKeysTable)
        keysLookup.localKeysTable!.writeBytes(compressedBytes);
      rootElement.writeBytes(compressedBytes, keysLookup);
      compressedBytes.trim();
      var compressed = zstd.encode(compressedBytes.asBytes());
      bytes.autoGrow = true;
      bytes.writeBytes(compressed);
      bytes.trim();
    }
    else {
      if (header.hasLocalKeysTable)
        keysLookup.localKeysTable!.writeBytes(bytes);
      rootElement.writeBytes(bytes, keysLookup);
    }
  }

  void updateSize() {
    header.hasLocalKeysTable = keysLookup.localKeysTable != null && keysLookup.localKeysTable!.count > 0;
  }

  @override
  int get size {
    var size = header.size;
    if (header.hasLocalKeysTable)
      size += keysLookup.localKeysTable!.size;
    size += rootElement.size;
    return size;
  }
}
