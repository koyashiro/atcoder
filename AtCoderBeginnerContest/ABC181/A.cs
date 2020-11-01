using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC181
{
    public class A
    {
        public static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());
            var colors = new string[2]
            {
                "White", "Black"
            };
            Console.WriteLine(colors[n % 2]);
        }
    }
}
