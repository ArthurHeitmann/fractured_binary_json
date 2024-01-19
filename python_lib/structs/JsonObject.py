from __future__ import annotations
from DataType import DataType, DataTypeChar
from FileWrapper import BinaryReader, BinaryWriter
from KeysLookup import KeysLookup
from SizeClass import SizeClass
from ioClass import JsonIoClass
import structs.Element as Element


_sizeClassToType = {
	SizeClass.empty: DataTypeChar.emptyObject,
	SizeClass.small: DataTypeChar.smallObject,
	SizeClass.big: DataTypeChar.bigObject,
	SizeClass.long: DataTypeChar.longObject
}
_typeToSizeClass = {
	DataTypeChar.emptyObject: SizeClass.empty,
	DataTypeChar.smallObject: SizeClass.small,
	DataTypeChar.bigObject: SizeClass.big,
	DataTypeChar.longObject: SizeClass.long
}

class JsonObject(JsonIoClass):
	length: int
	entries: list[JsonObjectEntry]
	sizeClass: SizeClass

	def __init__(self, length: int, entries: list[JsonObjectEntry], sizeClass: SizeClass):
		self.length = length
		self.entries = entries
		self.sizeClass = sizeClass

	@staticmethod
	def fromJson(json: object, keysLookup: KeysLookup) -> tuple[DataType, JsonObject|None]:
		if not isinstance(json, dict):
			raise Exception(f"JsonObject.fromJson: json is not a dict: {json}")
		object = json
		length = len(object)
		sizeClass = SizeClass.fromSize(length)
		type = _sizeClassToType[sizeClass]
		if type is None:
			raise Exception(f"JsonObject.fromJson: invalid size class: {sizeClass}")
		entries = [
			JsonObjectEntry.fromMapEntry(entry, keysLookup)
			for entry in object.items()
		]
		return (
			DataType(type),
			JsonObject(length, entries, sizeClass) if sizeClass != SizeClass.empty else None
		)
	
	@staticmethod
	def readBytes(bytes: BinaryReader, dataType: DataType, keysLookup: KeysLookup) -> JsonObject:
		sizeClass = _typeToSizeClass[dataType.char]
		if sizeClass is None:
			raise Exception(f"JsonObject.readBytes: dataType is not a object: {dataType}")
		length = sizeClass.readInt(bytes)
		entries = [
			JsonObjectEntry.readBytes(bytes, keysLookup)
			for _ in range(length)
		]
		return JsonObject(length, entries, sizeClass)
	
	def toJson(self) -> object:
		return {entry.key: entry.value.toJson() for entry in self.entries}
	
	def writeBytes(self, bytes: BinaryWriter, keysLookup: KeysLookup) -> None:
		self.sizeClass.writeInt(bytes, self.length)
		for entry in self.entries:
			entry.writeBytes(bytes, keysLookup)

	@property
	def size(self) -> int:
		size = self.sizeClass.bytes
		for entry in self.entries:
			size += entry.size
		return size
	
class JsonObjectEntry(JsonIoClass):
	key: str
	value: Element.Element

	def __init__(self, key: str, value: Element.Element):
		self.key = key
		self.value = value

	@staticmethod
	def fromMapEntry(entry: tuple[str, object], keysLookup: KeysLookup) -> JsonObjectEntry:
		key = entry[0]
		value = Element.Element.fromJson(entry[1], keysLookup)
		keysLookup.visitKey(key)
		return JsonObjectEntry(key, value)
	
	@staticmethod
	def readBytes(bytes: BinaryReader, keysLookup: KeysLookup) -> JsonObjectEntry:
		key = keysLookup.lookupIndex(bytes.readUint16())
		value = Element.Element.readBytes(bytes, keysLookup)
		return JsonObjectEntry(key, value)
	
	def toJson(self) -> tuple[str, object]:
		return (self.key, self.value.toJson())
	
	def writeBytes(self, bytes: BinaryWriter, keysLookup: KeysLookup) -> None:
		bytes.writeUint16(keysLookup.getKeyIndex(self.key))
		self.value.writeBytes(bytes, keysLookup)

	@property
	def size(self) -> int:
		return 2 + self.value.size
