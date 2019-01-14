#!/usr/bin/env rdmd
import std.algorithm, std.exception, std.format, std.range, std.stdio;

void main(string[] args) {
    enforce(args.length == 2, "Invalid args\n" ~
    "./tool <input.csv> <colum-name> <replacement-string> <output.csv>");

    auto inFile = args[1], columName = args[2], replacement = args[3], outFile = args[4];
    auto lines = File(inFile).byLine.map!(a => a.splitter(",").chunks(2));

    auto colIndex = lines.front.countUntil(columName);
    enforce(colIndex >= 0, "Invalid column. Valid columns: %(%s, %)".format(lines.front));

    auto os = File(outFile, "w");
    os.writefln("%-(%s,%)", lines.front);
    
    foreach (line; lines.dropOne) {
        os.writeln(line.enumerate // iterate with an (index, value) tuple
                       .map!(a => a.index == colIndex ? replacement : a.value)
                       .joiner(","));
    }
}