from __future__ import annotations
from DataType import DataType, DataTypeChar
from FileWrapper import BinaryReader, BinaryWriter
from KeysLookup import KeysLookup
from SizeClass import SizeClass
from ioClass import JsonIoClass
import structs.Element as Element


_sizeClassToType = {
	SizeClass.empty: DataTypeChar.emptyArray,
	SizeClass.small: DataTypeChar.smallArray,
	SizeClass.big: DataTypeChar.bigArray,
	SizeClass.long: DataTypeChar.longArray
}
_typeToSizeClass = {
	DataTypeChar.emptyArray: SizeClass.empty,
	DataTypeChar.smallArray: SizeClass.small,
	DataTypeChar.bigArray: SizeClass.big,
	DataTypeChar.longArray: SizeClass.long
}

class JsonArray(JsonIoClass):
	length: int
	values: list[Element.Element]
	sizeClass: SizeClass

	def __init__(self, length: int, values: list[Element.Element], sizeClass: SizeClass):
		self.length = length
		self.values = values
		self.sizeClass = sizeClass

	@staticmethod
	def fromJson(json: object, keysLookup: KeysLookup):
		if not isinstance(json, list):
			raise Exception(f"JsonArray.fromJson: json is not a List: {json}")
		array = json
		length = len(array)
		sizeClass = SizeClass.fromSize(length)
		type = _sizeClassToType[sizeClass]
		if type is None:
			raise Exception(f"JsonArray.fromJson: invalid size class: {sizeClass}")
		arrayValues = [Element.Element.fromJson(e, keysLookup) for e in array]
		return (
			DataType(type),
			JsonArray(length, arrayValues, sizeClass) if sizeClass != SizeClass.empty else None
		)
	
	@staticmethod
	def readBytes(bytes: BinaryReader, dataType: DataType, keysLookup: KeysLookup) -> JsonArray:
		sizeClass = _typeToSizeClass[dataType.char]
		if sizeClass is None:
			raise Exception(f"JsonArray.readBytes: dataType is not a array: {dataType}")
		length = sizeClass.readInt(bytes)
		arrayValues = [Element.Element.readBytes(bytes, keysLookup) for i in range(length)]
		return JsonArray(length, arrayValues, sizeClass)
	
	def toJson(self) -> object:
		return [e.toJson() for e in self.values]
	
	def writeBytes(self, bytes: BinaryWriter, keysLookup: KeysLookup):
		self.sizeClass.writeInt(bytes, self.length)
		for value in self.values:
			value.writeBytes(bytes, keysLookup)
	
	@property
	def size(self) -> int:
		size = self.sizeClass.bytes
		for value in self.values:
			size += value.size
		return size
