using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC179
{
    public class A
    {
        public static void Main(string[] args)
        {
            var s = Console.ReadLine().ToCharArray();
            if (s[^1] == 's')
            {
                Console.WriteLine($"{new string(s)}es");
            }
            else
            {
                Console.WriteLine($"{new string(s)}s");
            }
        }
    }
}
