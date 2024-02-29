from __future__ import annotations

from zstandard import ZstdCompressionDict, ZstdCompressor, ZstdDecompressor
from FileWrapper import BinaryReader, BinaryWriter, ByteReader, ByteWriter
from KeysLookup import KeysLookup
from ioClass import JsonIoClass
from structs.Element import Element
from structs.Header import Header
from structs.KeysTable import KeysTable


class FracturedJson(JsonIoClass):
	header: Header
	rootElement: Element
	keysLookup: KeysLookup
	_compressor: ZstdCompressor = ZstdCompressor()
	_decompressor: ZstdDecompressor = ZstdDecompressor()

	def __init__(self, header: Header, rootElement: Element, keysLookup: KeysLookup):
		self.header = header
		self.rootElement = rootElement
		self.keysLookup = keysLookup

	@staticmethod
	def fromJson(json: object, globalKeysTable: KeysTable) -> FracturedJson:
		keysLookup = KeysLookup(globalKeysTable)
		header = Header()
		rootElement = Element.fromJson(json, keysLookup)
		ret = FracturedJson(header, rootElement, keysLookup)
		ret.updateSize()
		return ret
	
	@classmethod
	def readBytes(cls, bytes: BinaryReader, globalKeysTable: KeysTable) -> FracturedJson:
		header = Header.readBytes(bytes)
		if header.useZstd:
			decompressed = cls._decompressor.decompress(bytes.readBytes(bytes.size - bytes.tell()))
			bytes = ByteReader(decompressed)
		localKeysTable: KeysTable|None = None
		if header.hasLocalKeysTable:
			localKeysTable = KeysTable.readBytes(bytes)
		keysLookup = KeysLookup(globalKeysTable, localKeysTable)
		rootElement = Element.readBytes(bytes, keysLookup)
		return FracturedJson(header, rootElement, keysLookup)
	
	def toJson(self) -> object:
		return self.rootElement.toJson()
	
	def writeBytes(self, bytes: BinaryWriter, keysLookup: KeysLookup|None = None, compressionLevel: int = 3, zstdDict: ZstdCompressionDict|None = None) -> None:
		keysLookup = keysLookup or self.keysLookup
		self.updateSize()
		self.header.writeBytes(bytes)
		if self.header.useZstd:
			uncompressedBytes = ByteWriter()
			if self.header.hasLocalKeysTable:
				keysLookup.localKeysTable.writeBytes(uncompressedBytes)
			self.rootElement.writeBytes(uncompressedBytes, keysLookup)
			compressedBytes = ZstdCompressor(level=compressionLevel, dict_data=zstdDict) \
				.compress(uncompressedBytes.byteData)
			bytes.writeBytes(compressedBytes)
		else:
			if self.header.hasLocalKeysTable:
				keysLookup.localKeysTable.writeBytes(bytes)
			self.rootElement.writeBytes(bytes, keysLookup)

	def updateSize(self) -> None:
		self.header.hasLocalKeysTable = self.keysLookup.localKeysTable is not None and self.keysLookup.localKeysTable.count > 0
	
	@property
	def size(self) -> int:
		size = self.header.size
		if self.header.hasLocalKeysTable:
			size += self.keysLookup.localKeysTable.size
		size += self.rootElement.size
		return size
