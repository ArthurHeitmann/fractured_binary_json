from __future__ import annotations
from typing import Callable
from DataType import DataType, DataTypeChar
from FileWrapper import BinaryReader, BinaryWriter
from KeysLookup import KeysLookup
from ioClass import JsonIoClass


_FloatReadWrite = tuple[Callable[[BinaryReader], float], Callable[[BinaryWriter, float], None], int]
_floatRW: _FloatReadWrite = (
	lambda bytes: bytes.readFloat(),
	lambda bytes, value: bytes.writeFloat(value),
	4
)
_doubleRW: _FloatReadWrite = (
	lambda bytes: bytes.readDouble(),
	lambda bytes, value: bytes.writeDouble(value),
	8
)
_dataTypeToRW: dict[DataTypeChar, _FloatReadWrite] = {
	DataTypeChar.float: _floatRW,
	DataTypeChar.double: _doubleRW
}
_largestSafeFloat = 16777216.0

class JsonFloat(JsonIoClass):
	value: float
	readWrite: _FloatReadWrite
	_size: int

	def __init__(self, value: float, readWrite: _FloatReadWrite, size: int):
		self.value = value
		self.readWrite = readWrite
		self._size = size

	@staticmethod
	def fromJson(json: object) -> tuple[DataType, JsonFloat|None]:
		if not isinstance(json, float):
			raise Exception(f"JsonFloat.fromJson: json is not an float: {json}")
		number = json
		result: tuple[DataTypeChar, _FloatReadWrite|None]
		if number == 0.0:
			result = (DataTypeChar.zeroFloat, None)
		elif number < _largestSafeFloat and number > -_largestSafeFloat:
			result = (DataTypeChar.float, _floatRW)
		else:
			result = (DataTypeChar.double, _doubleRW)

		return (
			DataType(result[0]),
			None if result[1] is None else JsonFloat(number, result[1], result[1][2])
		)

	@staticmethod
	def readBytes(bytes: BinaryReader, dataType: DataType) -> JsonFloat:
		floatRW = _dataTypeToRW[dataType.char]
		if floatRW is None:
			raise Exception(f"JsonFloat.readBytes: dataType is not an float: {dataType}")
		value = floatRW[0](bytes)
		return JsonFloat(value, floatRW, floatRW[2])

	def toJson(self) -> object:
		return self.value

	def writeBytes(self, bytes: BinaryWriter, keysLookup: KeysLookup) -> None:
		self.readWrite[1](bytes, self.value)
	
	@property
	def size(self) -> int:
		return self._size
