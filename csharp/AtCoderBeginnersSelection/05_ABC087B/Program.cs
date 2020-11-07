using System;
using System.Linq;

namespace ABC087B
{
    class Program
    {
        static void Main(string[] args)
        {
            var a = int.Parse(Console.ReadLine());
            var b = int.Parse(Console.ReadLine());
            var c = int.Parse(Console.ReadLine());
            var x = int.Parse(Console.ReadLine());

            var count = 0;

            for (int i = 0; i < a + 1; i++)
            {
                for (int j = 0; j < b + 1; j++)
                {
                    for (int k = 0; k < c + 1; k++)
                    {
                        if (x == 500 * i + 100 * j + 50 * k)
                        {
                            count++;
                        }
                    }
                }
            }

            Console.WriteLine(count);
        }
    }
}
