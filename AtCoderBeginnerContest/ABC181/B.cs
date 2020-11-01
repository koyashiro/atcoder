using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC181
{
    public class B
    {
        public static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());
            var abs = Enumerable.Range(0, n).Select(_ => Console.ReadLine().Split().Select(long.Parse).ToArray()).ToArray();
            var result = abs.Select(ab => (ab[0] + ab[1]) * (ab[1] - ab[0] + 1)).Sum() / 2d;
            Console.WriteLine(result);
        }
    }
}
