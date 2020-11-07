using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC055
{
    public class QuestionB
    {
        public static void Main(string[] args)
        {
            var n = long.Parse(Console.ReadLine());
            var m = (long)Math.Pow(10, 9) + 7;

            var result = 1L;
            for (int i = 0; i < n; i++)
            {
                result = result * (i + 1) % m;
            }

            Console.WriteLine(result);
        }
    }
}
