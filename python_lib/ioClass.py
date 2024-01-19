from DataType import DataType
from FileWrapper import BinaryReader, BinaryWriter
from KeysLookup import KeysLookup


class JsonIoClass:
	@staticmethod
	def fromJson(json: object, keysLookup: KeysLookup):
		raise NotImplementedError()

	@staticmethod
	def readBytes(bytes: BinaryReader, dataType: DataType, keysLookup: KeysLookup):
		raise NotImplementedError()

	def toJson(self) -> object:
		raise NotImplementedError()

	def writeBytes(self, bytes: BinaryWriter, keysLookup: KeysLookup):
		raise NotImplementedError()

	@property
	def size(self) -> int:
		raise NotImplementedError()
