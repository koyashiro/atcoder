using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC154
{
    public class QuestionA
    {
        public static void Main(string[] args)
        {
            var st = Console.ReadLine().Split();
            var ab = Console.ReadLine().Split().Select(int.Parse).ToArray();

            var (s, t) = (st[0], st[1]);
            var (a, b) = (ab[0], ab[1]);
            var u = Console.ReadLine();

            if (s == u)
            {
                a--;
            }
            else
            {
                b--;
            }

            Console.WriteLine($"{a} {b}");
        }
    }
}
