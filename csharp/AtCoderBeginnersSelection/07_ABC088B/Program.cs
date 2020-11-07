using System;
using System.Linq;
using System.Collections.Generic;

namespace ABC088B
{
    class Program
    {
        static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());
            var a = Console.ReadLine().Split().Select(int.Parse).ToList();
            a.Sort();
            a.Reverse();

            var queue = new Queue<int>(a);

            var alice = 0;
            var bob = 0;

            for (int i = 0; i < n; i++)
            {
                if (i % 2 == 0)
                {
                    alice += queue.Dequeue();
                }
                else
                {
                    bob += queue.Dequeue();
                }
            }

            Console.WriteLine(alice - bob);
        }
    }
}
