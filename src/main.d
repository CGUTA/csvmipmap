#!/usr/bin/env rdmd
import std.algorithm, std.exception, std.format, std.range, std.stdio;
import std.conv;
import std.stdio;
import std.range;
import std.algorithm.iteration;
void main(string[] args)
{
    auto chunkSize = 2;
    enforce(args.length == 2, "Invalid args\n" ~
    "./tool <input.csv>");

    auto inFile = args[1];
    
    //auto file = [[1.0, 2.0, 3.0, 4.0, 5.1],[1.0, 2.0, 3.0, 4.0, 5.1],[1.0, 2.0, 3.0, 4.0, 5.1]] ;

    //auto n = [[[2.2, 1, 1], [1.86454, 5, 1]], [[1, 1, 1.0]]] ;

    auto lines = File(inFile)
                    .byLine
                    .map!(a => a.splitter("\t")
                                .map!( a => to!float(a)))
                    .chunks(chunkSize);
    


    //writeln(lines);   

    auto n = lines
                .map!( a => a.map!( b => b.chunks(chunkSize)
                                        .map!( a => a.reduce!("a + b") )));
    //writeln(n);  


    auto result = n.map!( a=> a.front.walkLength.iota.map!(i => transversal(a, i)).map!( a => a.reduce!("a + b") ));
    writeln(result);

        
    /*
    foreach(e; result)
    {
        foreach (i; e)
        {
            writeln(i);
        }
    }*/


}