from structs.KeysTable import KeysTable


class KeysLookup:
	globalKeysTable: KeysTable
	localKeysTable: KeysTable|None

	def __init__(self, globalKeysTable: KeysTable, localKeysTable: KeysTable|None = None):
		self.globalKeysTable = globalKeysTable
		self.localKeysTable = localKeysTable

	def lookupIndex(self, index: int) -> str:
		if index <= self.globalKeysTable.count:
			return self.globalKeysTable.lookupIndex(index)
		else:
			if self.localKeysTable is None:
				raise Exception("Local keys table not found")
			return self.localKeysTable.lookupIndex(index - self.globalKeysTable.count)
		
	def getKeyIndex(self, key: str) -> int:
		index = self.globalKeysTable.getKeyIndex(key)
		if index is not None:
			return index
		if self.localKeysTable is None:
			self.localKeysTable = KeysTable()
		index = self.localKeysTable.getKeyIndex(key, allowCreate=True)
		if index is None:
			raise Exception("Could not create key in local keys table")
		return index + self.globalKeysTable.count
	
	def visitKey(self, key: str):
		self.getKeyIndex(key)
