using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC070
{
    public class QuestionB
    {
        public static void Main(string[] args)
        {
            var abcd = Console.ReadLine().Split().Select(int.Parse).ToArray();
            var a = abcd[0];
            var b = abcd[1];
            var c = abcd[2];
            var d = abcd[3];

            var lower = Math.Max(a, c);
            var upper = Math.Min(b, d);
            var result = Math.Max(0, upper - lower);
            Console.WriteLine(result);
        }
    }
}
