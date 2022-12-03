with open("./input.txt") as f:

    def part_one():
        total = 0
        for line in f.read().splitlines():
            first, second = set(line[: len(line) // 2]), set(line[len(line) // 2 :])
            common_char = (first & second).pop()
            if common_char.isupper():
                total += ord(common_char) - ord("A") + 27
            else:
                total += ord(common_char) - ord("a") + 1
        return total

    def part_two():
        total = 0
        lines = f.read().splitlines()
        for i in range(0, len(lines), 3):
            first, second, third = set(lines[i]), set(lines[i + 1]), set(lines[i + 2])
            common_char = (first & second & third).pop()
            if common_char.isupper():
                total += ord(common_char) - ord("A") + 27
            else:
                total += ord(common_char) - ord("a") + 1
        return total

    print(part_two())
