using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC154
{
    public class QuestionD
    {
        public static void Main(string[] args)
        {
            var nk = Console.ReadLine().Split().Select(int.Parse).ToArray();
            var (n, k) = (nk[0], nk[1]);
            var p = Console.ReadLine().Split().Select(int.Parse).ToArray();

            var pAverages = Enumerable.Range(0, n).Select(i => (p[i] - 1) * 0.5D + 1D).ToArray();

            var sums = new double[n];
            sums[0] = pAverages.Take(k).Sum();
            for (int i = 1; i < n - k + 1; i++)
            {
                sums[i] = sums[i - 1] - pAverages[i - 1] + pAverages[i - 1 + k];
            }
            
            Console.WriteLine(sums.Max());
        }
    }
}
