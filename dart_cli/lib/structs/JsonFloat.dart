
import "../../ByteDataWrapper.dart";
import "../../DataType.dart";
import "../../KeysLookup.dart";
import "../../ioClass.dart";

typedef FloatReadWrite = (double Function(ByteDataWrapper), void Function(ByteDataWrapper, double), int);
final FloatReadWrite _floatRW = (
  (bytes) => bytes.readFloat32(),
  (bytes, value) => bytes.writeFloat32(value),
  4
);
final FloatReadWrite _doubleRW = (
  (bytes) => bytes.readFloat64(),
  (bytes, value) => bytes.writeFloat64(value),
  8
);
final Map<DataTypeChar, FloatReadWrite> _dataTypeToRW = {
  DataTypeChar.float: _floatRW,
  DataTypeChar.double: _doubleRW
};
const _largestSafeFloat = 16777216.0;

class JsonFloat extends JsonIoClass {
  final double value;
  final FloatReadWrite readWrite;
  final int _size;

  const JsonFloat(this.value, this.readWrite, this._size);

  static (DataType, JsonFloat?) fromJson(Object? json) {
    if (json is! double)
      throw Exception("JsonFloat.fromJson: json is not an double: $json");
    var number = json;
    (DataTypeChar, FloatReadWrite?) result;
    if (number == 0.0)
      result = (DataTypeChar.zeroFloat, null);
    else if (number < _largestSafeFloat && number > -_largestSafeFloat)
      result = (DataTypeChar.float, _floatRW);
    else
      result = (DataTypeChar.double, _doubleRW);

    return (
      DataType(result.$1),
      result.$2 == null ? null : JsonFloat(number, result.$2!, result.$2!.$3)
    );
  }

  static JsonFloat readBytes(ByteDataWrapper bytes, DataType dataType) {
    var floatRW = _dataTypeToRW[dataType.char];
    if (floatRW == null)
      throw Exception("JsonFloat.readBytes: dataType is not an double: $dataType");
    double value = floatRW.$1(bytes);
    return JsonFloat(value, floatRW, floatRW.$3);
  }

  @override
  Object? toJson() {
    return value;
  }

  @override
  void writeBytes(ByteDataWrapper bytes, KeysLookup keysLookup) {
    readWrite.$2(bytes, value);
  }
  
  @override
  int get size => _size;
}
