using System;
using System.Linq;
using System.Collections.Generic;

namespace ABC049C
{
    class Program
    {
        static void Main(string[] args)
        {
            var input = Console.ReadLine().Reverse().ToList();

            while (input.Any())
            {
                if (input.Count >= 7 && input.Take(7).SequenceEqual("remaerd"))
                {
                    input.RemoveRange(0, 7);
                    continue;
                }

                if (input.Count >= 5 && input.Take(5).SequenceEqual("maerd"))
                {
                    input.RemoveRange(0, 5);
                    continue;
                }

                if (input.Count >= 6 && input.Take(6).SequenceEqual("resare"))
                {
                    input.RemoveRange(0, 6);
                    continue;
                }

                if (input.Count >= 5 && input.Take(5).SequenceEqual("esare"))
                {
                    input.RemoveRange(0, 5);
                    continue;
                }

                Console.WriteLine("NO");
                return;
            }

            Console.WriteLine("YES");
        }
    }
}
