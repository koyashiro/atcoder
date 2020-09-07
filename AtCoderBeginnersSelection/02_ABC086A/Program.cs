using System;
using System.Linq;

namespace ABC086A
{
    class Program
    {
        static void Main(string[] args)
        {
            var ab = Console.ReadLine().Split().Select(int.Parse).ToArray();
            var a = ab[0];
            var b = ab[1];

            if ((a * b) % 2 == 1)
            {
                Console.WriteLine("Odd");
            }
            else
            {
                Console.WriteLine("Even");
            }
        }
    }
}
