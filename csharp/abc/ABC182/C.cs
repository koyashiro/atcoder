using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC182
{
    public class C
    {
        public static void Main(string[] args)
        {
            var n = Console.ReadLine().Select(c => c - 48).ToList();
            var sum = n.Sum();
            var rem = sum % 3;
            var threes = n.Where(ni => ni % 3 == 0).ToArray();
            var twos = n.Where(ni => ni % 3 == 2).ToArray();
            var ones = n.Where(ni => ni % 3 == 1).ToArray();

            switch (rem)
            {
                case 0:
                    Console.WriteLine(0);
                    return;
                case 1:
                    if (ones.Any() && n.Count > 1)
                    {
                        Console.WriteLine(1);
                        return;
                    }
                    else if (twos.Length >= 2 && n.Count > 2)
                    {
                        Console.WriteLine(2);
                        return;
                    }
                    else
                    {
                        Console.WriteLine(-1);
                        return;
                    }
                case 2:
                    if (twos.Any() && n.Count > 1)
                    {
                        Console.WriteLine(1);
                        return;
                    }
                    else if (ones.Length >= 2 && n.Count > 2)
                    {
                        Console.WriteLine(2);
                        return;
                    }
                    else
                    {
                        Console.WriteLine(-1);
                        return;
                    }
            }
        }
    }
}
