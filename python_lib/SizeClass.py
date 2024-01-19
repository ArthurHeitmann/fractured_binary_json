from enum import Enum

from FileWrapper import BinaryReader, BinaryWriter


class SizeClass(Enum):
	empty = 0, 0
	small = 256, 1
	big = 65536, 2
	long = 4294967296, 4

	def __init__(self, size: int, bytes: int):
		self.size = size
		self.bytes = bytes

	@staticmethod
	def fromSize(size: int):
		if size == SizeClass.empty.size:
			return SizeClass.empty
		if size < SizeClass.small.size:
			return SizeClass.small
		if size < SizeClass.big.size:
			return SizeClass.big
		if size < SizeClass.long.size:
			return SizeClass.long
		raise Exception("Size $size is too big")
	
	def readInt(self, bytes: BinaryReader):
		if self == SizeClass.empty:
			return 0
		elif self == SizeClass.small:
			return bytes.readUint8()
		elif self == SizeClass.big:
			return bytes.readUint16()
		elif self == SizeClass.long:
			return bytes.readUint32()
		else:
			raise Exception("Unknown size class: " + str(self))
		
	def writeInt(self, bytes: BinaryWriter, value: int):
		if self == SizeClass.empty:
			pass
		elif self == SizeClass.small:
			bytes.writeUint8(value)
		elif self == SizeClass.big:
			bytes.writeUint16(value)
		elif self == SizeClass.long:
			bytes.writeUint32(value)
		else:
			raise Exception("Unknown size class: " + str(self))
