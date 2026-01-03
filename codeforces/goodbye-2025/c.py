t = int(input())
for _ in range(t):
    n = int(input())
    nice_arr = [int(item) for item in input().split()]
    prefix_sum = []
    sum = 0
    for i, item in enumerate(nice_arr):
        prefix_sum.append(sum)
        if i == 0:
            sum += item
        else:
            sum += abs(item)

    suffix_sum = []
    sum = 0
    for item in nice_arr[::-1]:
        suffix_sum.append(sum)
        sum += -item
    suffix_sum = suffix_sum[::-1]

    # print(prefix_sum, suffix_sum)

    max_val = None
    for i in range(len(nice_arr)):
        total_sum = prefix_sum[i] + suffix_sum[i]
        if max_val == None or max_val < total_sum:
            max_val = total_sum

    print(max_val)

