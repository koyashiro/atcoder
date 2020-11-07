using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC179
{
    public class C
    {
        public static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());

            var count = 0L;

            // a: 1 ~ n - 1
            for (int a = 1; a < n; a++)
            {
                for (int b = 1; b < n; b++)
                {
                    if (n - a * b > 0)
                    {
                        count++;
                    }

                    if (a * b > n)
                    {
                        break;
                    }
                }
            }

            Console.WriteLine(count);
        }
    }
}
