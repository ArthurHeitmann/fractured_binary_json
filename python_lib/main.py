import orjson as json
import os
from zstandard import ZstdCompressor, compress
from FileWrapper import FileWrapper
from generateGlobalKeysTable import generateGlobalKeysTableFromJson

from structs.FracturedJson import FracturedJson
from structs.KeysTable import KeysTable

def main():
	# testFolder = r"D:\Cloud\Documents\Programming\python\fractured_binary_json\python_lib\test\benchmark_canada"
	# testFolder = r"D:\Cloud\Documents\Programming\python\fractured_binary_json\python_lib\test\benchmark_citm"
	testFolder = r"D:\Cloud\Documents\Programming\python\fractured_binary_json\python_lib\test\benchmark_jeopardy"
	obj = getObj()
	zstLevel = 3
	# raw
	path = os.path.join(testFolder, "1_raw.json")
	with open(path, "wb") as f:
		f.write(json.dumps(obj))
	# zst
	path = os.path.join(testFolder, f"2_zst{zstLevel}.zst")
	with open(path, "wb") as f:
		f.write(ZstdCompressor(zstLevel).compress(json.dumps(obj)))
	# frac json no global keys table
	path = os.path.join(testFolder, "3_frac_json_standalone.frac_json")
	fracJson = FracturedJson.fromJson(obj, KeysTable())
	fracJson.header.useZstd = False
	with FileWrapper.fromFile(path, "wb") as f:
		fracJson.writeBytes(f)
	# frac json no global keys table, zst
	fracJson.header.useZstd = True
	path = os.path.join(testFolder, f"4_frac_json_standalone_zst{zstLevel}.frac_json")
	with FileWrapper.fromFile(path, "wb") as f:
		fracJson.writeBytes(f, compressionLevel=zstLevel)
	# global keys table
	path = os.path.join(testFolder, "global_keys_table.frac_json_table")
	generateGlobalKeysTableFromJson([obj], path)
	with FileWrapper.fromFile(path, "rb") as f:
		keysTable = KeysTable.readBytes(f)
	# frac json with global keys table
	path = os.path.join(testFolder, "5_frac_json.frac_json")
	fracJson = FracturedJson.fromJson(obj, keysTable)
	fracJson.header.useZstd = False
	with FileWrapper.fromFile(path, "wb") as f:
		fracJson.writeBytes(f)
	# frac json with global keys table, zst
	fracJson.header.useZstd = True
	path = os.path.join(testFolder, f"6_frac_json_zst{zstLevel}.frac_json")
	with FileWrapper.fromFile(path, "wb") as f:
		fracJson.writeBytes(f, compressionLevel=zstLevel)


def getOneObj() -> dict:
	path = r"X:\reddit\intermediate\comments_trailing\comments_trailing_2023-08-20.jsonl"
	with open(path, "rb") as f:
		line = f.readline()
		obj = json.loads(line)
	deleteKeys = [
		# "body", "selftext", "title", "url",
	]
	for key in deleteKeys:
		if key in obj:
			del obj[key]
	return obj

def getObj():
	# path = r"D:\Cloud\Documents\Programming\python\fractured_binary_json\python_lib\test\benchmark_canada\canada.json"
	# path = r"D:\Cloud\Documents\Programming\python\fractured_binary_json\python_lib\test\benchmark_twitter\twitter.json"
	# path = r"D:\Cloud\Documents\Programming\python\fractured_binary_json\python_lib\test\benchmark_citm\citm_catalog.json"
	path = r"D:\Cloud\Documents\Programming\python\fractured_binary_json\python_lib\test\benchmark_jeopardy\jeopardy.json"
	with open(path, "rb") as f:
		obj = json.loads(f.read())
	return obj

if __name__ == '__main__':
	main()
