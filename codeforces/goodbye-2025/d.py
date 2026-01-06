def zero_out(a):
    hysteria = []
    # Attack from earlier to last until it is depleted
    last = a[-1][1]
    index = len(a) - 2;
    while index > -1:
        if a[index][1] < last:
            # Attack from index to last
            hysteria.append((a[index][0], a[-1][0]))
            last -= a[index][1]
        else:
            break
        index -= 1
    if last > 0 and index == -1:
        return [] # Impossible
    for i in range(0, index):
        # Attack from next to current
        hysteria.append((a[i+1][0], a[i][0]))
    hysteria.append((a[-1][0], a[index][0]))

    return hysteria


def mass_hysteria(a):
    if m > n // 2:
        return []

    if m == 0:
        return zero_out(a)
    
    hysteria = []
    
    # Attack inside leftover
    rem = n % m
    for i in range(0, rem):
        # Attack from i+1 to i
        hysteria.append((a[i+1][0], a[i][0]))

    # For everything else, attack while skipping by m
    for i in range(rem, n - m): 
        hysteria.append((a[i + m][0], a[i][0]))

    return hysteria

def print_hysteria(hysteria):
    if len(hysteria) == 0:
        print("-1")
    else:
        print(len(hysteria))
        for item in hysteria:
            print(f"{item[0]+1} {item[1]+1}")


t = int(input())
for _ in range(t):
    n, m = input().split()
    n, m = int(n), int(m)

    a = [int(item) for item in input().split()]

    # Convert to index
    a = list(enumerate(a))
    a.sort(key=lambda x: x[1])

    # print(a)

    hysteria = mass_hysteria(a)

    print_hysteria(hysteria)
        
