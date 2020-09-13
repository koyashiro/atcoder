using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC178
{
    public class QuestionB
    {
        public static void Main(string[] args)
        {
            var abcd = Console.ReadLine().Split().Select(long.Parse).ToArray();
            var (a, b, c, d) = (abcd[0], abcd[1], abcd[2], abcd[3]);
            var max = new long[4] { a * c, a * d, b * c, b * d }.Max();
            Console.WriteLine(max);
        }
    }
}
