with open('input') as f:
    sum = 0
    for l in f:
        first, last = '', ''
        for i in range(len(l)):
            if not first and l[i].isdigit():
                first = l[i]
            if not last and l[-i - 1].isdigit():
                last = l[-i - 1]
            if first and last:
                break
        sum += int(first + last)

print(sum)