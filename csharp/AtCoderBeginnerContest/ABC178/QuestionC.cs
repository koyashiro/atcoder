using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC178
{
    public class QuestionC
    {
        public static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());

            if (n == 1)
            {
                Console.WriteLine(0);
                return;
            }

            if (n == 2)
            {
                Console.WriteLine(2);
                return;
            }

            var m = 1000000007L;

            var countAll = 1L;
            var countIgnore0Or9 = 1L;
            var countIgnore0And9 = 1L;
            for (int i = 0; i < n; i++)
            {
                countIgnore0Or9 = (countIgnore0Or9 * 9L) % m;
                countIgnore0And9 = (countIgnore0And9 * 8L) % m;
                countAll = (countAll * 10L) % m;
            }

            var count = (((countAll + countIgnore0And9 - (2 * countIgnore0Or9)) % m) + m) % m;

            Console.WriteLine(count);
        }
    }
}
