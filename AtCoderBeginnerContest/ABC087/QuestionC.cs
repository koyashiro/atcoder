using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC087
{
    public class QuestionC
    {
        public static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());
            var a = Enumerable.Range(0, 2).Select(i => Console.ReadLine().Split().Select(int.Parse).ToArray()).ToArray();
            var max = Enumerable.Range(0, n).Select(i => Enumerable.Range(0, i).Select(x => a[0][x]).Sum() + a[0][i] + a[1][i] + Enumerable.Range(i + 1, n - i - 1).Select(x => a[1][x]).Sum()).Max();
            Console.WriteLine(max);
        }
    }
}
