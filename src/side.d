import std.stdio;
import std.range;
import std.algorithm.iteration;
void main()
{
    writeln("Hello D");
    
    auto line = [[1.0, 2.0, 3.0, 4.0, 5.1],[1.0, 2.0, 3.0, 4.0, 5.1],[1.0, 2.0, 3.0, 4.0, 5.1]] ;
    
    auto lines = line.chunks(2);
        
    auto n = lines.map!( a => a.map!( b => b.chunks(2).map!( a => a.reduce!("a + b") )));
    
    auto result = n.map!( a=> a.front.walkLength.iota.map!(i => transversal(a, i)).map!( a => a.reduce!("a + b") ));
        
    
    foreach(e; result)
    {
        foreach (i; e)
        {
        	writeln(i);
        }
    }
        

}