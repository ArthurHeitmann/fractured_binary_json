from __future__ import annotations
from DataType import DataType, DataTypeChar
from FileWrapper import BinaryReader, BinaryWriter
from KeysLookup import KeysLookup
from SizeClass import SizeClass
from ioClass import JsonIoClass

_sizeClassToType = {
	SizeClass.empty: DataTypeChar.emptyString,
	SizeClass.small: DataTypeChar.smallString,
	SizeClass.big: DataTypeChar.bigString,
	SizeClass.long: DataTypeChar.longString
}
_typeToSizeClass = {
	DataTypeChar.emptyString: SizeClass.empty,
	DataTypeChar.smallString: SizeClass.small,
	DataTypeChar.bigString: SizeClass.big,
	DataTypeChar.longString: SizeClass.long
}

class JsonString(JsonIoClass):
	length: int
	value: str
	sizeClass: SizeClass

	def __init__(self, length: int, value: str, sizeClass: SizeClass):
		self.length = length
		self.value = value
		self.sizeClass = sizeClass

	@staticmethod
	def fromJson(json: object) -> tuple[DataType, JsonString|None]:
		if not isinstance(json, str):
			raise Exception(f"JsonString.fromJson: json is not a String: {json}")
		string = json
		length = len(string.encode())
		sizeClass = SizeClass.fromSize(length)
		type = _sizeClassToType[sizeClass]
		if type is None:
			raise Exception(f"JsonString.fromJson: invalid size class: {sizeClass}")
		return (
			DataType(type),
			JsonString(length, string, sizeClass) if sizeClass is not SizeClass.empty else None
		)
	
	@staticmethod
	def readBytes(bytes: BinaryReader, dataType: DataType) -> JsonString:
		sizeClass = _typeToSizeClass[dataType.char]
		if sizeClass is None:
			raise Exception(f"JsonString.readBytes: dataType is not a string: {dataType}")
		length = sizeClass.readInt(bytes)
		value = bytes.readString(length)
		return JsonString(length, value, sizeClass)
	
	def toJson(self) -> object:
		return self.value
	
	def writeBytes(self, bytes: BinaryWriter, keysLookup: KeysLookup) -> None:
		self.sizeClass.writeInt(bytes, self.length)
		bytes.writeString(self.value)
	
	@property
	def size(self) -> int:
		return self.sizeClass.bytes + self.length
