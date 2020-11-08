using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC098
{
    public class QuestionC
    {
        public static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());
            var s = Console.ReadLine().AsSpan();

            var sumW = new int[n + 1];
            for (int i = 0; i < n; i++)
            {
                sumW[i + 1] = sumW[i] + (s[i] == 'W' ? 1 : 0);
            }

            var sumE = new int[n + 1];
            for (int i = 0; i < n; i++)
            {
                sumE[i + 1] = sumE[i] + (s[i] == 'E' ? 1 : 0);
            }

            var min = Enumerable.Range(0, n).Select(i => sumW[i] + (sumE[n] - sumE[i + 1])).Min();
            Console.WriteLine(min);
        }
    }
}
