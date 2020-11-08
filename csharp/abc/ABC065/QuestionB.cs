using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC065
{
    public class QuestionB
    {
        public static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());
            var a = Enumerable.Range(0, n).Select(i => int.Parse(Console.ReadLine())).ToArray();
            var hashSet = new HashSet<int>();

            var i = 0;
            while (true)
            {
                if (!hashSet.Add(i))
                {
                    Console.WriteLine(-1);
                    return;
                }

                i = a[i] - 1;

                if (i == 1)
                {
                    Console.WriteLine(hashSet.Count);
                    return;
                }
            }
        }
    }
}
