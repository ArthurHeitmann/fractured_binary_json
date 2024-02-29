from structs.KeysTable import KeysTable

_globalTableEnd = 0x8000
_localTableOffset = 0x8001

class KeysLookup:
	globalKeysTable: KeysTable
	localKeysTable: KeysTable|None

	def __init__(self, globalKeysTable: KeysTable, localKeysTable: KeysTable|None = None):
		self.globalKeysTable = globalKeysTable
		self.localKeysTable = localKeysTable

	def lookupIndex(self, index: int) -> str:
		if index <= _globalTableEnd:
			return self.globalKeysTable.lookupIndex(index)
		else:
			if self.localKeysTable is None:
				raise Exception("Local keys table not found")
			return self.localKeysTable.lookupIndex(index - _localTableOffset)
		
	def getKeyIndex(self, key: str) -> int:
		index = self.globalKeysTable.getKeyIndex(key)
		if index is not None:
			return index
		if self.localKeysTable is None:
			self.localKeysTable = KeysTable()
		index = self.localKeysTable.getKeyIndex(key, allowCreate=True)
		if index is None:
			raise Exception("Could not create key in local keys table")
		return index + _localTableOffset
	
	def visitKey(self, key: str):
		self.getKeyIndex(key)
