
import "../ByteDataWrapper.dart";
import "../ioClass.dart";

class KeyMapping extends IoClass {
  late final int index;
  late final int keyLength;
  late final String keyName;

  KeyMapping(this.index, this.keyName) :
    keyLength = keyName.length;
  
  KeyMapping.readBytes(ByteDataWrapper bytes) {
    index = bytes.readUint16();
    keyLength = bytes.readUint8();
    keyName = bytes.readString(keyLength);
  }

  @override
  void writeBytes(ByteDataWrapper bytes) {
    bytes.writeUint16(index);
    bytes.writeUint8(keyLength);
    bytes.writeString(keyName);
  }

  @override
  int get size => 3 + keyLength;
}
