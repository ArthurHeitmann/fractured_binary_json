from __future__ import annotations
from DataType import DataType, DataTypeChar
from FileWrapper import BinaryReader, BinaryWriter
from KeysLookup import KeysLookup
from ioClass import JsonIoClass
from structs.JsonArray import JsonArray
from structs.JsonFloat import JsonFloat
from structs.JsonInt import JsonInt
from structs.JsonObject import JsonObject
from structs.JsonString import JsonString


class Element(JsonIoClass):
	dataType: DataType
	value: JsonIoClass|None

	def __init__(self, dataType: DataType, value: JsonIoClass|None):
		self.dataType = dataType
		self.value = value

	@staticmethod
	def fromJson(json: object, keysLookup: KeysLookup) -> Element:
		result: tuple[DataType, JsonIoClass|None]
		if isinstance(json, int):
			result = JsonInt.fromJson(json)
		elif isinstance(json, float):
			result = JsonFloat.fromJson(json)
		elif isinstance(json, bool):
			result = (
				DataType(DataTypeChar.trueBool if json else DataTypeChar.falseBool),
				None
			)
		elif json is None:
			result = (DataType(DataTypeChar.nullValue), None)
		elif isinstance(json, list):
			result = JsonArray.fromJson(json, keysLookup)
		elif isinstance(json, dict):
			result = JsonObject.fromJson(json, keysLookup)
		elif isinstance(json, str):
			result = JsonString.fromJson(json)
		else:
			raise Exception(f"Element.fromJson: json is not a valid type: {json}")
		
		return Element(result[0], result[1])
	
	@staticmethod
	def readBytes(bytes: BinaryReader, keysLookup: KeysLookup) -> Element:
		dataType = DataType.readBytes(bytes)
		value = None
		if not dataType.char.hasJsonValue:
			value = Element._parseValue(dataType, bytes, keysLookup)
		return Element(dataType, value)
	
	@staticmethod
	def _parseValue(dataType: DataType, bytes: BinaryReader, keysLookup: KeysLookup) -> JsonIoClass:
		if dataType.char in [DataTypeChar.smallObject, DataTypeChar.bigObject, DataTypeChar.longObject]:
			return JsonObject.readBytes(bytes, dataType, keysLookup)
		elif dataType.char in [DataTypeChar.smallArray, DataTypeChar.bigArray, DataTypeChar.longArray]:
			return JsonArray.readBytes(bytes, dataType, keysLookup)
		elif dataType.char in [DataTypeChar.smallString, DataTypeChar.bigString, DataTypeChar.longString]:
			return JsonString.readBytes(bytes, dataType)
		elif dataType.char in [DataTypeChar.int8, DataTypeChar.uint8, DataTypeChar.int16, DataTypeChar.uint16, DataTypeChar.int32, DataTypeChar.uint32, DataTypeChar.int64, DataTypeChar.uint64]:
			return JsonInt.readBytes(bytes, dataType)
		elif dataType.char in [DataTypeChar.float, DataTypeChar.double]:
			return JsonFloat.readBytes(bytes, dataType)
		else:
			raise Exception(f"Element._parseValue: unexpected data type: {dataType.char}")
		
	def toJson(self) -> object:
		if self.dataType.char.hasJsonValue:
			return self.dataType.char.jsonValue
		return self.value.toJson()
	
	def writeBytes(self, bytes: BinaryWriter, keysLookup: KeysLookup):
		self.dataType.writeBytes(bytes)
		if self.value is not None:
			self.value.writeBytes(bytes, keysLookup)

	@property
	def size(self) -> int:
		size = self.dataType.size
		if self.value is not None:
			size += self.value.size
		return size
