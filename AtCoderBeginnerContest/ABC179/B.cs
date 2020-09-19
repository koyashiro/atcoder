using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC179
{
    public class B
    {
        public static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());
            var d = Enumerable.Range(0, n).Select(_ => Console.ReadLine().Split().Select(Convert.ToChar).ToArray()).ToArray();

            int duplicatedCount = 0;
            for (int i = 0; i < n; i++)
            {
                if (d[i][0] == d[i][1])
                {
                    duplicatedCount++;
                }
                else
                {
                    duplicatedCount = 0;
                }

                if (duplicatedCount == 3)
                {
                    break;
                }
            }

            if (duplicatedCount == 3)
            {
                Console.WriteLine("Yes");
            }
            else
            {
                Console.WriteLine("No");
            }
        }
    }
}
