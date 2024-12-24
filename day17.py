import z3

s = z3.Solver()
a = z3.BitVec("a", 16 * 3 + 3)

prog = [2, 4, 1, 6, 7, 5, 4, 6, 1, 4, 5, 5, 0, 3, 3, 0]

for i in range(16):
    t_a = a >> (i * 3)
    b = t_a & 0b111
    b = b ^ 6
    c = t_a >> b
    b = b ^ c
    b = b ^ 4
    b = b & 0b111
    s.add( b == prog[i] )

s.add( a >> (16 * 3) == 0 )

if s.check() == z3.sat:
    print(s.model().eval(a))
