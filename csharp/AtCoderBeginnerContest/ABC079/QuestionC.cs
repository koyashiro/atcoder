using System;
using System.Collections.Generic;
using System.Linq;

namespace ABC079
{
    public class QuestionC
    {
        public static void Main(string[] args)
        {
            var abcd = Console.ReadLine().Select(i => (int)i - 48).ToArray();
            var (a, b, c, d) = (abcd[0], abcd[1], abcd[2], abcd[3]);

            var ops = new (bool op1, bool op2, bool op3)[8]
            {
                (false, false, true),
                (false, false, false),
                (false, true, true),
                (false, true, false),
                (true, false, true),
                (true, false, false),
                (true, true, true),
                (true, true, false)
            };

            foreach (var op in ops)
            {
                var result = Calculate(Calculate(Calculate(a, op.op1, b), op.op2, c), op.op3, d);
                if (result == 7)
                {
                    var op1 = op.op1 ? '+' : '-';
                    var op2 = op.op2 ? '+' : '-';
                    var op3 = op.op3 ? '+' : '-';
                    Console.WriteLine($"{a}{op1}{b}{op2}{c}{op3}{d}=7");
                    return;
                }
            }
        }

        private static int Calculate(int left, bool op, int right)
        {
            if (op)
            {
                return left + right;
            }
            else{
                return left - right;
            }
        }
    }
}
