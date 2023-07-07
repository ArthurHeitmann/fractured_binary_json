
import "ByteDataWrapper.dart";
import "KeysLookup.dart";

abstract class IoClass {
  const IoClass();
  IoClass.readBytes(ByteDataWrapper bytes);
  void writeBytes(ByteDataWrapper bytes);

  int get size;
}
abstract class JsonIoClass {
  const JsonIoClass();
  JsonIoClass.fromJson(Object? json);
  JsonIoClass.readBytes(ByteDataWrapper bytes, KeysLookup keysLookup);
  
  Object? toJson();
  void writeBytes(ByteDataWrapper bytes, KeysLookup keysLookup);
  
  int get size;
}
