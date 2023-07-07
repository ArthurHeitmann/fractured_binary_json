import "dart:convert";
import "dart:io";

import "package:args/args.dart";
import "package:fractured_binary_json_format/ByteDataWrapper.dart";
import "package:fractured_binary_json_format/CliOptions.dart";
import "package:fractured_binary_json_format/generateGlobalKeysTable.dart";
import "package:fractured_binary_json_format/structs/FracturedJson.dart";
import "package:fractured_binary_json_format/structs/KeysTable.dart";

void main(List<String> arguments) async {
  var argParser = ArgParser();
  argParser.addCommand("toJson");
  argParser.addCommand("toBinary");
  argParser.addCommand("generateGlobalKeysTable");
  argParser.addOption("globalKeysTable", abbr: "g");
  argParser.addOption("input", abbr: "i");
  argParser.addOption("output", abbr: "o");
  argParser.addMultiOption("index", abbr: "x");
  var argResults = argParser.parse(arguments);
  var options = CliOptions(
    globalKeysTablePath: argResults["globalKeysTable"],
    inputPath: argResults["input"],
    outputPath: argResults["output"],
    indexFiles: argResults["index"],
    toJson: argResults.command?.name == "toJson",
    toBinary: argResults.command?.name == "toBinary",
    generateGlobalKeysTable: argResults.command?.name == "generateGlobalKeysTable",
  );
  if (!options.onlyOneCommandSpecified)
    throw Exception("Exactly one command must be specified");
  if (options.toJson || options.toBinary) {
    if (options.inputPath == null)
      throw Exception("Input path must be specified");
    if (options.outputPath == null) {
      if (options.toJson)
        options.outputPath = "${options.inputPath}.json";
      else
        options.outputPath = "${options.inputPath}.frac_json";
    }
  }
  else if (options.generateGlobalKeysTable) {
    if (options.indexFiles == null || options.indexFiles!.isEmpty)
      throw Exception("Index files must be specified");
  }

  if (options.toJson)
    await convertToJson(options);
  else if (options.toBinary)
    await convertToBinary(options);
  else if (options.generateGlobalKeysTable)
    await generateGlobalKeysTable(options);
  else
    throw Exception("Unknown command");
}

Future<void> convertToJson(CliOptions options) async {
  var globalKeysTableBytes = await ByteDataWrapper.fromFile(options.globalKeysTablePath);
  var globalKeysTable = KeysTable.readBytes(globalKeysTableBytes);
  var inputBytes = await ByteDataWrapper.fromFile(options.inputPath!);
  var fracturedJson = FracturedJson.readBytes(inputBytes, globalKeysTable);
  var json = fracturedJson.toJson();
  var jsonStr = JsonEncoder().convert(json);
  await File(options.outputPath!).writeAsString(jsonStr);
}

Future<void> convertToBinary(CliOptions options) async {
  var globalKeysTableBytes = await ByteDataWrapper.fromFile(options.globalKeysTablePath);
  var globalKeysTable = KeysTable.readBytes(globalKeysTableBytes);
  var inputJsonStr = await File(options.inputPath!).readAsString();
  var inputJson = JsonDecoder().convert(inputJsonStr);
  var fracturedJson = FracturedJson.fromJson(inputJson, globalKeysTable);
  var outputBytes = ByteDataWrapper.allocate(fracturedJson.size);
  fracturedJson.writeBytes(outputBytes);
  await outputBytes.toFile(options.outputPath!);
}

Future<void> generateGlobalKeysTable(CliOptions options) async {
  await generateGlobalKeysTableFromFiles(options.indexFiles!, options.globalKeysTablePath);
}
