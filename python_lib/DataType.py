from __future__ import annotations
from enum import Enum

from FileWrapper import BinaryReader, BinaryWriter
from SizeClass import SizeClass


class DataTypeChar(Enum):
	emptyObject = "o"
	smallObject = "O"
	bigObject = "p"
	longObject = "P"
	emptyArray = "a"
	smallArray = "A"
	bigArray = "c"
	longArray = "C"
	emptyString = "s"
	smallString = "S"
	bigString = "t"
	longString = "T"
	zeroInt = "0"
	int8 = "i"
	uint8 = "I"
	int16 = "j"
	uint16 = "J"
	int32 = "k"
	uint32 = "K"
	int64 = "l"
	uint64 = "L"
	zeroFloat = "f"
	float = "F"
	double = "d"
	falseBool = "b"
	trueBool = "B"
	nullValue = "z"

	@property
	def char(self):
		return self.value

	@staticmethod
	def fromChar(c: str):
		for dataType in DataTypeChar:
			if dataType.value == c:
				return dataType
		raise Exception("Unknown data type char: " + c)
	
	@property
	def sizeClass(self):
		if self in (DataTypeChar.emptyObject, DataTypeChar.emptyArray, DataTypeChar.emptyString):
			return SizeClass.empty
		elif self in (DataTypeChar.smallObject, DataTypeChar.smallArray, DataTypeChar.smallString):
			return SizeClass.small
		elif self in (DataTypeChar.bigObject, DataTypeChar.bigArray, DataTypeChar.bigString):
			return SizeClass.big
		elif self in (DataTypeChar.longObject, DataTypeChar.longArray, DataTypeChar.longString):
			return SizeClass.long
		else:
			raise Exception("Unknown data type char: " + self.value)
	
	@property
	def hasJsonValue(self):
		return self in {
			DataTypeChar.emptyObject,
			DataTypeChar.emptyArray,
			DataTypeChar.emptyString,
			DataTypeChar.zeroInt,
			DataTypeChar.zeroFloat,
			DataTypeChar.falseBool,
			DataTypeChar.trueBool,
			DataTypeChar.nullValue,
		}
	
	@property
	def jsonValue(self):
		if self == DataTypeChar.emptyObject:
			return {}
		elif self == DataTypeChar.emptyArray:
			return []
		elif self == DataTypeChar.emptyString:
			return ""
		elif self == DataTypeChar.zeroInt:
			return 0
		elif self == DataTypeChar.zeroFloat:
			return 0.0
		elif self == DataTypeChar.falseBool:
			return False
		elif self == DataTypeChar.trueBool:
			return True
		elif self == DataTypeChar.nullValue:
			return None
		else:
			raise Exception("Data type char " + self.value + " has no json value")
	
class DataType:
	char: DataTypeChar

	def __init__(self, char: DataTypeChar):
		self.char = char
	
	@staticmethod
	def readBytes(bytes: BinaryReader) -> DataType:
		char = DataTypeChar.fromChar(bytes.readString(1))
		return DataType(char)
	
	def writeBytes(self, bytes: BinaryWriter):
		bytes.writeString(self.char.value)
	
	@property
	def size(self) -> int:
		return 1
