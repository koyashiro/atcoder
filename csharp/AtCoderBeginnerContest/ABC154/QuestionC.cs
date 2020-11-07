using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC154
{
    public class QuestionC
    {
        public static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());
            var a = Console.ReadLine().Split().Select(long.Parse);

            if (a.GroupBy(x => x).Any(g => g.Count() > 1))
            {
                Console.WriteLine("NO");
            }
            else
            {
                Console.WriteLine("YES");
            }
        }
    }
}
