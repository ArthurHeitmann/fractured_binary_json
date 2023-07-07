
import "dart:convert";
import "dart:io";
import "dart:typed_data";

class ByteDataWrapper {
  ByteBuffer _buffer;
  late ByteData _data;
  final int length;
  final Endian endian = Endian.little;
  bool autoGrow;
  int _position = 0;
  
  ByteDataWrapper(this._buffer, {this.autoGrow = false}) :
    length = _buffer.lengthInBytes {
    _data = _buffer.asByteData(0, _buffer.lengthInBytes);
    _position = 0;
  }

  ByteDataWrapper.allocate(this.length, {this.autoGrow = false}) : 
    _buffer = ByteData(length).buffer,
    _position = 0 {
    _data = _buffer.asByteData(0, _buffer.lengthInBytes);
  }

  static Future<ByteDataWrapper> fromFile(String path) async {
    const twoGB = 2 * 1024 * 1024 * 1024;
    var fileSize = await File(path).length();
    if (fileSize < twoGB) {
      var buffer = await File(path).readAsBytes();
      return ByteDataWrapper(buffer.buffer);
    } else {
      var buffer = Uint8List(fileSize).buffer;
      var file = File(path).openRead();
      int position = 0;
      await for (var bytes in file) {
        buffer.asUint8List().setRange(position, position + bytes.length, bytes);
        position += bytes.length;
      }
      return ByteDataWrapper(buffer);
    }
  }

  Future<void> toFile(String path) async {
    await File(path).writeAsBytes(asBytes());
  }

  int get position => _position;

  set position(int value) {
    if (value < 0 || value > length)
      throw RangeError.range(value, 0, _data.lengthInBytes, "View size");
    if (value > _buffer.lengthInBytes)
      throw RangeError.range(value, 0, _buffer.lengthInBytes, "Buffer size");
    
    _position = value;
  }

  _expand() {
    if (autoGrow) {
      var newBuffer = ByteData(_buffer.lengthInBytes * 2).buffer;
      newBuffer.asUint8List().setRange(0, _buffer.lengthInBytes, _buffer.asUint8List());
      _buffer = newBuffer;
      _data = _buffer.asByteData();
    } else {
      throw Exception("Buffer overflow");
    }
  }

  _checkSize(int size) {
    if (_position + size > _buffer.lengthInBytes) {
      _expand();
    }
  }

  trim() {
    if (_position == _buffer.lengthInBytes)
      return;
    var newBuffer = ByteData(_position).buffer;
    newBuffer.asUint8List().setRange(0, _position, _buffer.asUint8List());
    _buffer = newBuffer;
    _data = _buffer.asByteData();
  }

  double readFloat32() {
    var value = _data.getFloat32(_position, endian);
    _position += 4;
    return value;
  }

  double readFloat64() {
    var value = _data.getFloat64(_position, endian);
    _position += 8;
    return value;
  }

  int readInt8() {
    var value = _data.getInt8(_position);
    _position += 1;
    return value;
  }

  int readInt16() {
    var value = _data.getInt16(_position, endian);
    _position += 2;
    return value;
  }

  int readInt32() {
    var value = _data.getInt32(_position, endian);
    _position += 4;
    return value;
  }

  int readInt64() {
    var value = _data.getInt64(_position, endian);
    _position += 8;
    return value;
  }

  int readUint8() {
    var value = _data.getUint8(_position);
    _position += 1;
    return value;
  }

  int readUint16() {
    var value = _data.getUint16(_position, endian);
    _position += 2;
    return value;
  }

  int readUint32() {
    var value = _data.getUint32(_position, endian);
    _position += 4;
    return value;
  }

  int readUint64() {
    var value = _data.getUint64(_position, endian);
    _position += 8;
    return value;
  }

  List<double> readFloat32List(int length) {
    var list = List<double>.generate(length, (_) => readFloat32());
    return list;
  }

  List<double> readFloat64List(int length) {
    var list = List<double>.generate(length, (_) => readFloat64());
    return list;
  }

  List<int> readInt8List(int length) {
    return List<int>.generate(length, (_) => readInt8());
  }

  List<int> readInt16List(int length) {
    return List<int>.generate(length, (_) => readInt16());
  }

  List<int> readInt32List(int length) {
    return List<int>.generate(length, (_) => readInt32());
  }

  List<int> readInt64List(int length) {
    return List<int>.generate(length, (_) => readInt64());
  }

  List<int> readUint8List(int length) {
    return List<int>.generate(length, (_) => readUint8());
  }

  List<int> readUint16List(int length) {
    return List<int>.generate(length, (_) => readUint16());
  }

  List<int> readUint32List(int length) {
    return List<int>.generate(length, (_) => readUint32());
  }

  List<int> readUint64List(int length) {
    return List<int>.generate(length, (_) => readUint64());
  }

  Uint8List asBytes() {
    return _buffer.asUint8List();
  }

  Uint8List asUint8List(int length) {
    var list = Uint8List.view(_buffer, _position, length);
    _position += length;
    return list;
  }

  Uint16List asUint16List(int length) {
    var list = Uint16List.view(_buffer, _position, length);
    _position += length * 2;
    return list;
  }

  Uint32List asUint32List(int length) {
    var list = Uint32List.view(_buffer, _position, length);
    _position += length * 4;
    return list;
  }

  Uint64List asUint64List(int length) {
    var list = Uint64List.view(_buffer, _position, length);
    _position += length * 8;
    return list;
  }

  Int8List asInt8List(int length) {
    var list = Int8List.view(_buffer, _position, length);
    _position += length;
    return list;
  }

  Int16List asInt16List(int length) {
    var list = Int16List.view(_buffer, _position, length);
    _position += length * 2;
    return list;
  }

  Int32List asInt32List(int length) {
    var list = Int32List.view(_buffer, _position, length);
    _position += length * 4;
    return list;
  }

  String readString(int length) {
    List<int> bytes = asUint8List(length);
    return utf8.decode(bytes, allowMalformed: true);
  }


  void writeFloat32(double value) {
    _checkSize(4);
    _data.setFloat32(_position, value, endian);
    _position += 4;
  }

  void writeFloat64(double value) {
    _checkSize(8);
    _data.setFloat64(_position, value, endian);
    _position += 8;
  }

  void writeInt8(int value) {
    _checkSize(1);
    _data.setInt8(_position, value);
    _position += 1;
  }

  void writeInt16(int value) {
    _checkSize(2);
    _data.setInt16(_position, value, endian);
    _position += 2;
  }

  void writeInt32(int value) {
    _checkSize(4);
    _data.setInt32(_position, value, endian);
    _position += 4;
  }

  void writeInt64(int value) {
    _checkSize(8);
    _data.setInt64(_position, value, endian);
    _position += 8;
  }

  void writeUint8(int value) {
    _checkSize(1);
    _data.setUint8(_position, value);
    _position += 1;
  }

  void writeUint16(int value) {
    _checkSize(2);
    _data.setUint16(_position, value, endian);
    _position += 2;
  }

  void writeUint32(int value) {
    _checkSize(4);
    _data.setUint32(_position, value, endian);
    _position += 4;
  }

  void writeUint64(int value) {
    _checkSize(8);
    _data.setUint64(_position, value, endian);
    _position += 8;
  }

  void writeString(String value) {
    var bytes = encodeString(value);
    writeBytes(bytes);
  }

  void writeBytes(List<int> bytes) {
    _checkSize(bytes.length);
    _buffer.asUint8List().setRange(_position, _position + bytes.length, bytes);
    _position += bytes.length;
  }

  static List<int> encodeString(String value) {
    return utf8.encode(value);
  }
}
