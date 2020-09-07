using System;
using System.Linq;

namespace PracticeA
{
    class Program
    {
        static void Main(string[] args)
        {
            var a = int.Parse(Console.ReadLine());
            var bc = Console.ReadLine().Split().Select(int.Parse).ToArray();
            var b = bc[0];
            var c = bc[1];
            var s = Console.ReadLine();
            Console.WriteLine($"{a + b + c} {s}");
        }
    }
}
