using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC182
{
    public class A
    {
        public static void Main(string[] args)
        {
            var ab = Console.ReadLine().Split().Select(long.Parse).ToArray();
            var (a, b) = (ab[0], ab[1]);

            var maxFollow = 2 * a + 100;
            var result = maxFollow - b;

            Console.WriteLine(result);
        }
    }
}
