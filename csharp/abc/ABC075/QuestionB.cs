using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC075
{
    public class QuestionB
    {
        public static void Main(string[] args)
        {
            var hw = Console.ReadLine().Split().Select(int.Parse).ToArray();
            var h = hw[0];
            var w = hw[1];
            var s = Enumerable.Range(0, h).Select(i => Console.ReadLine().ToArray()).ToArray();

            var adjacents = new (int dx, int dy)[]
            {
                (-1, -1), (0, -1), (1, -1),
                (-1, 0), (0, 0), (1, 0),
                (-1, 1), (0, 1), (1, 1),
            };

            for (int i = 0; i < h; i++)
            {
                for (int j = 0; j < w; j++)
                {
                    if (s[i][j] == '#') continue;

                    var count = 0;
                    foreach (var adjacent in adjacents)
                    {
                        var x = i + adjacent.dx;
                        if (x < 0 || h <= x) continue;
                        var y = j + adjacent.dy;
                        if (y < 0 || w <= y) continue;

                        if (s[x][y] == '#') count++;
                    }

                    s[i][j] = (char)(count + 48);
                }
            }

            foreach (var si in s)
            {
                Console.WriteLine(new string(si));
            }
        }
    }
}
