from __future__ import annotations
from typing import Callable
from DataType import DataType, DataTypeChar
from FileWrapper import BinaryReader, BinaryWriter
from KeysLookup import KeysLookup
from ioClass import JsonIoClass


_IntRange = tuple[int, int]
_int8Range = (-128, 127)
_uint8Range = (0, 255)
_int16Range = (-32768, 32767)
_uint16Range = (0, 65535)
_int32Range = (-2147483648, 2147483647)
_uint32Range = (0, 4294967295)
_int64Range = (-9223372036854775808, 9223372036854775807)

IntReadWrite = tuple[Callable[[BinaryReader], int], Callable[[BinaryWriter, int], None], int]
_int8RW = (
  lambda bytes: bytes.readInt8(),
  lambda bytes, value: bytes.writeInt8(value),
  8
)
_uint8RW = (
  lambda bytes: bytes.readUint8(),
  lambda bytes, value: bytes.writeUint8(value),
  8
)
_int16RW = (
  lambda bytes: bytes.readInt16(),
  lambda bytes, value: bytes.writeInt16(value),
  16
)
_uint16RW = (
  lambda bytes: bytes.readUint16(),
  lambda bytes, value: bytes.writeUint16(value),
  16
)
_int32RW = (
  lambda bytes: bytes.readInt32(),
  lambda bytes, value: bytes.writeInt32(value),
  32
)
_uint32RW = (
  lambda bytes: bytes.readUint32(),
  lambda bytes, value: bytes.writeUint32(value),
  32
)
_int64RW = (
  lambda bytes: bytes.readInt64(),
  lambda bytes, value: bytes.writeInt64(value),
  32
)
_uint64RW = (
  lambda bytes: bytes.readUint64(),
  lambda bytes, value: bytes.writeUint64(value),
  32
)
_dataTypeToRW = {
  DataTypeChar.int8: _int8RW,
  DataTypeChar.uint8: _uint8RW,
  DataTypeChar.int16: _int16RW,
  DataTypeChar.uint16: _uint16RW,
  DataTypeChar.int32: _int32RW,
  DataTypeChar.uint32: _uint32RW,
  DataTypeChar.int64: _int64RW,
  DataTypeChar.uint64: _uint64RW
}

class JsonInt(JsonIoClass):
	value: int
	readWrite: IntReadWrite
	_size: int

	def __init__(self, value: int, readWrite: IntReadWrite, size: int):
		self.value = value
		self.readWrite = readWrite
		self._size = size

	@staticmethod
	def fromJson(json: object) -> tuple[DataType, JsonInt|None]:
		if not isinstance(json, int):
			raise Exception(f"JsonInt.fromJson: json is not an int: {json}")
		number = json
		result: tuple[DataTypeChar, IntReadWrite|None]
		if number == 0:
			result = (DataTypeChar.zeroInt, None)
		elif number < 0:
			if number >= _int8Range[0]:
				result = (DataTypeChar.int8, _int8RW)
			elif number >= _int16Range[0]:
				result = (DataTypeChar.int16, _int16RW)
			elif number >= _int32Range[0]:
				result = (DataTypeChar.int32, _int32RW)
			elif number >= _int64Range[0]:
				result = (DataTypeChar.int64, _int64RW)
			else:
				raise Exception(f"JsonInt.fromJson: number is too small: {number}")
		else:
			if number <= _uint8Range[1]:
				result = (DataTypeChar.uint8, _uint8RW)
			elif number <= _uint16Range[1]:
				result = (DataTypeChar.uint16, _uint16RW)
			elif number <= _uint32Range[1]:
				result = (DataTypeChar.uint32, _uint32RW)
			else:
				result = (DataTypeChar.uint64, _uint64RW)

		return (
			DataType(result[0]),
			None if result[1] is None else JsonInt(number, result[1], result[1][2])
		)

	@staticmethod
	def readBytes(bytes: BinaryReader, dataType: DataType) -> JsonInt:
		intRW = _dataTypeToRW[dataType.char]
		if intRW is None:
			raise Exception(f"JsonInt.readBytes: dataType is not an int: {dataType}")
		value = intRW[0](bytes)
		return JsonInt(value, intRW, intRW[2])

	def toJson(self) -> object:
		return self.value

	def writeBytes(self, bytes: BinaryWriter, keysLookup: KeysLookup) -> None:
		self.readWrite[1](bytes, self.value)

	@property
	def size(self) -> int:
		return self._size
