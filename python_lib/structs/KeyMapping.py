from FileWrapper import BinaryReader, BinaryWriter


class KeyMapping:
	index: int
	keyLength: int
	keyName: str

	def __init__(self, index: int, keyName: str):
		self.index = index
		self.keyName = keyName
		self.keyLength = len(keyName)

	@staticmethod
	def readBytes(bytes: BinaryReader):
		index = bytes.readUint16()
		keyLength = bytes.readUint8()
		keyName = bytes.readString(keyLength)
		return KeyMapping(index, keyName)

	def writeBytes(self, bytes: BinaryWriter):
		bytes.writeUint16(self.index)
		bytes.writeUint8(self.keyLength)
		bytes.writeString(self.keyName)

	@property
	def size(self) -> int:
		return 3 + self.keyLength
