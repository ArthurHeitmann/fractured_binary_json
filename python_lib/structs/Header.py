from FileWrapper import BinaryReader, BinaryWriter


_magic = "FJ"
_defaultUseZstd = True

class Header:
	magic: str
	config: int

	def __init__(self):
		self.magic = _magic
		self.config = 0
		self.useZstd = _defaultUseZstd

	@staticmethod
	def readBytes(bytes: BinaryReader):
		magic = bytes.readString(2)
		config = bytes.readUint8()
		if magic != _magic:
			raise Exception("Invalid magic")
		return Header()

	def writeBytes(self, bytes: BinaryWriter):
		bytes.writeString(self.magic)
		bytes.writeUint8(self.config)

	@property
	def hasLocalKeysTable(self) -> bool:
		return (self.config & 0x10) != 0

	@hasLocalKeysTable.setter
	def hasLocalKeysTable(self, value: bool):
		if value:
			self.config |= 0x10
		else:
			self.config &= ~0x10

	@property
	def useZstd(self) -> bool:
		return (self.config & 0x20) != 0

	@useZstd.setter
	def useZstd(self, value: bool):
		if value:
			self.config |= 0x20
		else:
			self.config &= ~0x20

	@property
	def version(self) -> int:
		return self.config & 0x0F

	@version.setter
	def version(self, value: int):
		if value < 0 or value > 0x0F:
			raise Exception("Version must be in range 0..15")
		self.config &= ~0x0F
		self.config |= value

	@property
	def size(self) -> int:
		return 3