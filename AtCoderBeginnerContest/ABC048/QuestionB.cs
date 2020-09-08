using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC048
{
    public class QuestionB
    {
        public static void Main(string[] args)
        {
            var abx = Console.ReadLine().Split().Select(long.Parse).ToArray();
            var (a, b, x) = (abx[0], abx[1], abx[2]);

            var count = F(b, x) - F(a - 1L, x);

            Console.WriteLine(count);
        }

        private static long F(long n, long x)
        {
            if (n >= 0)
            {
                return n / x + 1;
            }
            else
            {
                return 0;
            }
        }
    }
}
