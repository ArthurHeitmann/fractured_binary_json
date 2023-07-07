
class CliOptions {
  final String globalKeysTablePath;
  final String? inputPath;
  String? outputPath;
  final List<String>? indexFiles;
  final bool toJson;
  final bool toBinary;
  final bool generateGlobalKeysTable;

  CliOptions({
    required this.globalKeysTablePath,
    this.inputPath,
    this.outputPath,
    this.indexFiles,
    this.toJson = false,
    this.toBinary = false,
    this.generateGlobalKeysTable = false,
  });

  bool get onlyOneCommandSpecified {
    var count = 0;
    if (toJson)
      count++;
    if (toBinary)
      count++;
    if (generateGlobalKeysTable)
      count++;
    return count == 1;
  }
}
