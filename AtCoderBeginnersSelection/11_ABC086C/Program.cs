using System;
using System.Linq;
using System.Collections.Generic;

namespace ABC086C
{
    class Program
    {
        static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());
            var plan = Enumerable.Range(0, n).Select(i =>
            {
                var input = Console.ReadLine().Split().Select(int.Parse).ToArray();
                return (t: input[0], x: input[1], y: input[2]);
            }).ToList();

            plan.Insert(0, (0, 0, 0));

            for (int i = 0; i < plan.Count - 1; i++)
            {
                var prev = plan[i];
                var next = plan[i + 1];

                var dt = next.t - prev.t;
                var dist = Math.Abs(next.x - prev.x) + Math.Abs(next.y - prev.y);

                if (dt < dist || dt % 2 != dist % 2)
                {
                    Console.WriteLine("No");
                    return;
                }
            }

            Console.WriteLine("Yes");
        }
    }
}
