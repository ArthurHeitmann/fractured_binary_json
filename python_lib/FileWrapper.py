import os
from struct import pack, unpack
from typing import BinaryIO, Union

class BinaryReader:
	size: int

	def tell(self) -> int:
		raise NotImplementedError

	def readUint8(self) -> int:
		raise NotImplementedError
	
	def readInt8(self) -> int:
		raise NotImplementedError
	
	def readUint16(self) -> int:
		raise NotImplementedError
	
	def readInt16(self) -> int:
		raise NotImplementedError
	
	def readUint32(self) -> int:
		raise NotImplementedError
	
	def readInt32(self) -> int:
		raise NotImplementedError
	
	def readUint64(self) -> int:
		raise NotImplementedError
	
	def readInt64(self) -> int:
		raise NotImplementedError
	
	def readFloat(self) -> float:
		raise NotImplementedError
	
	def readDouble(self) -> float:
		raise NotImplementedError
	
	def readString(self, length: int) -> str:
		raise NotImplementedError
	
	def readBytes(self, length: int) -> bytes:
		raise NotImplementedError

class BinaryWriter(BinaryReader):
	def writeUint8(self, value: int):
		raise NotImplementedError

	def writeInt8(self, value: int):
		raise NotImplementedError

	def writeUint16(self, value: int):
		raise NotImplementedError
	
	def writeInt16(self, value: int):
		raise NotImplementedError
	
	def writeUint32(self, value: int):
		raise NotImplementedError

	def writeInt32(self, value: int):
		raise NotImplementedError

	def writeUint64(self, value: int):
		raise NotImplementedError

	def writeInt64(self, value: int):
		raise NotImplementedError

	def writeFloat(self, value: float):
		raise NotImplementedError

	def writeDouble(self, value: float):
		raise NotImplementedError

	def writeString(self, value: str):
		raise NotImplementedError

	def writeBytes(self, value: bytes):
		raise NotImplementedError

class FileWrapper(BinaryWriter):
	f: BinaryIO

	def __init__(self, f: BinaryIO, size: int):
		self.f = f
		self.size = size

	@staticmethod
	def fromFile(path: str, mode: str):
		f = open(path, mode) # type: ignore
		size = os.path.getsize(path)
		return FileWrapper(f, size) # type: ignore
	
	def __enter__(self):
		return self
	
	def __exit__(self, exc_type, exc_value, traceback):
		self.close()

	def close(self):
		self.f.close()
	
	def tell(self) -> int:
		return self.f.tell()

	def readUint8(self) -> int:
		return unpack('<B', self.f.read(1))[0]
	
	def readInt8(self) -> int:
		return unpack('<b', self.f.read(1))[0]
	
	def readUint16(self) -> int:
		return unpack('<H', self.f.read(2))[0]
	
	def readInt16(self) -> int:
		return unpack('<h', self.f.read(2))[0]
	
	def readUint32(self) -> int:
		return unpack('<I', self.f.read(4))[0]
	
	def readInt32(self) -> int:
		return unpack('<i', self.f.read(4))[0]
	
	def readUint64(self) -> int:
		return unpack('<Q', self.f.read(8))[0]
	
	def readInt64(self) -> int:
		return unpack('<q', self.f.read(8))[0]
	
	def readFloat(self) -> float:
		return unpack('<f', self.f.read(4))[0]
	
	def readDouble(self) -> float:
		return unpack('<d', self.f.read(8))[0]
	
	def readString(self, length: int) -> str:
		return self.f.read(length).decode('utf-8')
	
	def readBytes(self, length: int) -> bytes:
		return self.f.read(length)
	
	def writeUint8(self, value: int):
		self.f.write(pack('<B', value))

	def writeInt8(self, value: int):
		self.f.write(pack('<b', value))

	def writeUint16(self, value: int):
		self.f.write(pack('<H', value))
	
	def writeInt16(self, value: int):
		self.f.write(pack('<h', value))
	
	def writeUint32(self, value: int):
		self.f.write(pack('<I', value))

	def writeInt32(self, value: int):
		self.f.write(pack('<i', value))

	def writeUint64(self, value: int):
		self.f.write(pack('<Q', value))

	def writeInt64(self, value: int):
		self.f.write(pack('<q', value))

	def writeFloat(self, value: float):
		self.f.write(pack('<f', value))

	def writeDouble(self, value: float):
		self.f.write(pack('<d', value))

	def writeString(self, value: str):
		self.f.write(value.encode('utf-8'))

	def writeBytes(self, value: bytes):
		self.f.write(value)

class ByteReader(BinaryReader):
	_bytes: bytes
	pos: int

	def __init__(self, byteData: bytes):
		self._bytes = byteData
		self.pos = 0
		self.size = len(byteData)

	def _read(self, length: int) -> bytes:
		result = self._bytes[self.pos:self.pos + length]
		self.pos += length
		return result

	def tell(self) -> int:
		return self.pos

	def readUint8(self) -> int:
		return unpack('<B', self._read(1))[0]
	
	def readInt8(self) -> int:
		return unpack('<b', self._read(1))[0]
	
	def readUint16(self) -> int:
		return unpack('<H', self._read(2))[0]
	
	def readInt16(self) -> int:
		return unpack('<h', self._read(2))[0]
	
	def readUint32(self) -> int:
		return unpack('<I', self._read(4))[0]
	
	def readInt32(self) -> int:
		return unpack('<i', self._read(4))[0]
	
	def readUint64(self) -> int:
		return unpack('<Q', self._read(8))[0]
	
	def readInt64(self) -> int:
		return unpack('<q', self._read(8))[0]
	
	def readFloat(self) -> float:
		return unpack('<f', self._read(4))[0]
	
	def readDouble(self) -> float:
		return unpack('<d', self._read(8))[0]
	
	def readString(self, length: int) -> str:
		return self._read(length).decode('utf-8')
	
	def readBytes(self, length: int) -> bytes:
		return self._read(length)

class ByteWriter(BinaryWriter):
	byteData: bytearray

	def __init__(self):
		self.byteData = bytearray()

	def _write(self, value: bytes):
		self.byteData.extend(value)
	
	def writeUint8(self, value: int):
		self._write(pack('<B', value))

	def writeInt8(self, value: int):
		self._write(pack('<b', value))

	def writeUint16(self, value: int):
		self._write(pack('<H', value))
	
	def writeInt16(self, value: int):
		self._write(pack('<h', value))
	
	def writeUint32(self, value: int):
		self._write(pack('<I', value))

	def writeInt32(self, value: int):
		self._write(pack('<i', value))

	def writeUint64(self, value: int):
		self._write(pack('<Q', value))

	def writeInt64(self, value: int):
		self._write(pack('<q', value))

	def writeFloat(self, value: float):
		self._write(pack('<f', value))

	def writeDouble(self, value: float):
		self._write(pack('<d', value))

	def writeString(self, value: str):
		self._write(value.encode('utf-8'))

	def writeBytes(self, value: bytes):
		self._write(value)

	def asBytes(self) -> bytes:
		return bytes(self.byteData)
