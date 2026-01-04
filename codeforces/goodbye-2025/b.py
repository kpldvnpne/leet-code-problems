t = int(input())
for _ in range(t):
    r = input()
    prev_s = False
    n_ops = 0
    for i, c in enumerate(r):
        if c == 'u':
            if i == 0 or i == len(r) - 1:
                prev_s = True
                n_ops += 1
            elif prev_s == True:
                prev_s = False
            else:
                prev_s = True
                n_ops += 1
        else:
            prev_s = True
    print(n_ops)
    