import os
from typing import Iterable

from FileWrapper import FileWrapper
from structs.KeysTable import KeysTable


def generateGlobalKeysTableFromJson(objects: Iterable[dict], globalKeysTablePath: str, append: bool = False):
	if append and os.path.exists(globalKeysTablePath):
		with FileWrapper.fromFile(globalKeysTablePath, "rb") as f:
			keysTable = KeysTable.readBytes(f)
	else:
		keysTable = KeysTable()

	for obj in objects:
		_visitJson(obj, keysTable)

	with FileWrapper.fromFile(globalKeysTablePath, "wb") as f:
		keysTable.writeBytes(f)

def _visitJson(json, keysTable: KeysTable):
	if isinstance(json, dict):
		for key in json.keys():
			keysTable.visitKey(key)
		for value in json.values():
			_visitJson(value, keysTable)
	if isinstance(json, list):
		for item in json:
			_visitJson(item, keysTable)

def generateGlobalKeysTableFromList(keys: Iterable[str], globalKeysTablePath: str, append: bool = False):
	if append and os.path.exists(globalKeysTablePath):
		with FileWrapper.fromFile(globalKeysTablePath, "rb") as f:
			keysTable = KeysTable.readBytes(f)
	else:
		keysTable = KeysTable()

	for key in keys:
		keysTable.visitKey(key)

	with FileWrapper.fromFile(globalKeysTablePath, "wb") as f:
		keysTable.writeBytes(f)
