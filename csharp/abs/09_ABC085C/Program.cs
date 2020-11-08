using System;
using System.Linq;
using System.Collections.Generic;

namespace ABC085C
{
    class Program
    {
        static void Main(string[] args)
        {
            var ny = Console.ReadLine().Split().Select(int.Parse).ToArray();
            var n = ny[0];
            var y = ny[1];

            for (int i = 0; i < n + 1; i++)
            {
                for (int j = 0; j < n - i + 1; j++)
                {
                    var k = n - i - j;

                    if (i * 10000 + j * 5000 + k * 1000 == y)
                    {
                        Console.WriteLine($"{i} {j} {k}");
                        return;
                    }
                }
            }

            Console.WriteLine("-1 -1 -1");
        }
    }
}
