using System;
using System.Linq;

namespace ABC081B
{
    class Program
    {
        static void Main(string[] args)
        {
            var n = int.Parse(Console.ReadLine());
            var a = Console.ReadLine().Split().Select(int.Parse).ToArray();

            var count = 0;

            while (true)
            {
                if (!a.All(ai => ai % 2 == 0))
                {
                    break;
                }

                count++;
                a = a.Select(ai => ai / 2).ToArray();
            }

            Console.WriteLine(count);
        }
    }
}
