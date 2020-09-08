using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC060
{
    public class QuestionB
    {
        public static void Main(string[] args)
        {
            var abc = Console.ReadLine().Split().Select(int.Parse).ToArray();
            var (a, b, c) = (abc[0], abc[1], abc[2]);

            for (int i = 0; i < b; i++)
            {
                if (a * i % b == c)
                {
                    Console.WriteLine("YES");
                    return;
                }
            }

            Console.WriteLine("NO");
        }
    }
}
