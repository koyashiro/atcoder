using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC046
{
    public class QuestionB
    {
        public static void Main(string[] args)
        {
            var nk = Console.ReadLine().Split().Select(long.Parse).ToArray();
            var n = nk[0];
            var k = nk[1];

            var result = k;
            for (int i = 0; i < n - 1; i++)
            {
                result *= (k - 1);
            }

            Console.WriteLine(result);
        }
    }
}
