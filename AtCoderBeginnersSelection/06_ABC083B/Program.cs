using System;
using System.Linq;

namespace ABC083B
{
    class Program
    {
        static void Main(string[] args)
        {
            var nab = Console.ReadLine().Split().Select(int.Parse).ToArray();
            var n = nab[0];
            var a = nab[1];
            var b = nab[2];

            var total = 0;

            foreach (var number in Enumerable.Range(1, n).ToArray())
            {
                var sum = Enumerable.Range(0, 5).Select(i => (number / (int)Math.Pow(10, i)) % 10).Sum();

                if (a <= sum && sum <= b)
                {
                    total += number;
                }
            }

            Console.WriteLine(total);
        }
    }
}
