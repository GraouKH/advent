import timeit

with open('../inputs/input') as f:
    content = f.read()
    modulo = content.find('\n') + 1

    numbers = []
    number = ''
    look_around = [-1, 1, -modulo - 1, -modulo, -modulo + 1, modulo - 1, modulo, modulo + 1]
    is_good_digit = False
    for i, c in enumerate(content):
        if c.isdigit():
            number += c
            for l in look_around:
                if 0 < i + l < len(content) and not content[i + l].isdigit() and not content[i + l] in [".", "\n"]:
                    is_good_digit = True
        elif number:
            if is_good_digit:
                numbers.append(number)
            number = ''
            is_good_digit = False
    print(sum([int(i) for i in numbers])) 