from FileWrapper import BinaryReader, BinaryWriter
from structs.KeyMapping import KeyMapping


class KeysTable:
	count: int
	mappings: list[KeyMapping]

	def __init__(self):
		self.count = 0
		self.mappings = []

	@staticmethod
	def readBytes(bytes: BinaryReader):
		keysTable = KeysTable()
		keysTable.count = bytes.readUint16()
		keysTable.mappings = [KeyMapping.readBytes(bytes) for _ in range(keysTable.count)]
		return keysTable

	def writeBytes(self, bytes: BinaryWriter):
		bytes.writeUint16(self.count)
		for mapping in self.mappings:
			mapping.writeBytes(bytes)

	@property
	def size(self) -> int:
		size = 2
		for mapping in self.mappings:
			size += mapping.size
		return size

	def lookupIndex(self, index: int) -> str:
		return self.mappings[index].keyName

	def getKeyIndex(self, key: str, allowCreate: bool = False) -> int|None:
		for i in range(len(self.mappings)):
			if self.mappings[i].keyName == key:
				return i
		if not allowCreate:
			return None
		if len(self.mappings) >= 65536:
			raise Exception("KeysTable.getKeyIndex: too many keys")
		self.mappings.append(KeyMapping(len(self.mappings), key))
		self.count += 1
		return len(self.mappings) - 1

	def visitKey(self, key: str):
		self.getKeyIndex(key, allowCreate=True)
