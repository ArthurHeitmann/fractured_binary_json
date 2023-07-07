
import "../ByteDataWrapper.dart";
import "../ioClass.dart";

const _magic = "FJ";
const _defaultUseZstd = true;

class Header extends IoClass {
  String magic;
  int config;

  Header() :
    magic = _magic,
    config = 0 {
    useZstd = _defaultUseZstd;
  }

  Header.readBytes(ByteDataWrapper bytes) :
    magic = bytes.readString(2),
    config = bytes.readUint8() {
    if (magic != _magic)
      throw Exception("Invalid magic");
  }

  @override
  void writeBytes(ByteDataWrapper bytes) {
    bytes.writeString(magic);
    bytes.writeUint8(config);
  }

  bool get hasLocalKeysTable => (config & 0x10) != 0;

  set hasLocalKeysTable(bool value) {
    if (value)
      config |= 0x10;
    else
      config &= ~0x10;
  }

  bool get useZstd => (config & 0x20) != 0;

  set useZstd(bool value) {
    if (value)
      config |= 0x20;
    else
      config &= ~0x20;
  }

  int get version => config & 0x0F;

  set version(int value) {
    if (value < 0 || value > 0x0F)
      throw RangeError.range(value, 0, 0x0F, "Version");
    config &= ~0x0F;
    config |= value;
  }
  
  @override
  int get size => 3;
}
