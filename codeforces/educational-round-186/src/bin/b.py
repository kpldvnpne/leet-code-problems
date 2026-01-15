def find_max_layers(a, b):
    n_layers = 0
    required = 1
    while True:
        if a < required:
            break
        a -= required
        n_layers += 1

        required *= 2


        if b < required:
            break
        b -= required
        n_layers += 1

        required *= 2

    return n_layers

t = int(input())
for _ in range(t):
    a, b = input().split(' ')
    a, b = int(a), int(b)

    n_layers = max(find_max_layers(a, b), find_max_layers(b, a))

    print(n_layers)

