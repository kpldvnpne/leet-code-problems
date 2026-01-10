t = int(input())
for _ in range(t):
    n = int(input())
    a = [int(a_i) for a_i in input().split()]
    b = [int(b_i) for b_i in input().split()]
    c = [int(c_i) for c_i in input().split()]

    b_c_changes = 0
    for diff in range(n):
        possible = True
        for i in range(n):
            if not b[i] < c[i - diff]:
                possible = False
                break
        if possible:
            b_c_changes += 1
    
    a_b_changes = 0
    for diff in range(n):
        possible = True
        for i in range(n):
            if not a[i] < b[i - diff]:
                possible = False
                break

        if possible:
            a_b_changes += 1
    
    total_changes = n * a_b_changes * b_c_changes

    print(f"{total_changes}")