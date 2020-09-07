using System;
using System.Linq;
using System.Collections.Generic;

namespace ABC085B
{
    class Program
    {
        static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());
            var d = Enumerable.Range(0, n).Select(i => int.Parse(Console.ReadLine())).ToArray();

            Console.WriteLine(d.Distinct().Count());
        }
    }
}
