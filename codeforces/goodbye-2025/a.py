t = int(input())
for _ in range(t):
    s: str = input()
    n_y = 0
    for c in s:
        if c == 'Y':
            n_y += 1
            if n_y >= 2:
                break
    
    if n_y < 2:
        print("YES")
    else:
        print("NO")
