using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC182
{
    public class B
    {
        public static void Main(string[] args)
        {
            var n = long.Parse(Console.ReadLine());
            var a = Console.ReadLine().Split().Select(long.Parse).ToArray();
            var max = a.Max();

            var count = 0;
            var gcd = 0L;
            for (int i = 0; i < max - 1; i++)
            {
                var mod = max - i;
                var modCount = a.Count(ai => ai % mod == 0);
                if (count < modCount)
                {
                    count = modCount;
                    gcd = mod;
                }
            }

            Console.WriteLine(gcd);
        }
    }
}
