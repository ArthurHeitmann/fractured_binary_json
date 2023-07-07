import "ByteDataWrapper.dart";

enum SizeClass {
  empty(0, 0),
  small(256, 1),
  big(65536, 2),
  long(4294967296, 4);

  final int size;
  final int bytes;

  const SizeClass(this.size, this.bytes);

  static SizeClass fromSize(int size) {
    if (size == SizeClass.empty.size)
      return SizeClass.empty;
    if (size < SizeClass.small.size)
      return SizeClass.small;
    if (size < SizeClass.big.size)
      return SizeClass.big;
    if (size < SizeClass.long.size)
      return SizeClass.long;
    throw Exception("Size $size is too big");
  }

  int readInt(ByteDataWrapper bytes) {
    switch (this) {
      case SizeClass.empty:
        return 0;
      case SizeClass.small:
        return bytes.readUint8();
      case SizeClass.big:
        return bytes.readUint16();
      case SizeClass.long:
        return bytes.readUint32();
      default:
        throw Exception("Unknown size class: $this");
    }
  }

  void writeInt(ByteDataWrapper bytes, int value) {
    switch (this) {
      case SizeClass.empty:
        break;
      case SizeClass.small:
        bytes.writeUint8(value);
        break;
      case SizeClass.big:
        bytes.writeUint16(value);
        break;
      case SizeClass.long:
        bytes.writeUint32(value);
        break;
      default:
        throw Exception("Unknown size class: $this");
    }
  }
}
