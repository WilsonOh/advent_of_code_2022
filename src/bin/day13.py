from ast import literal_eval
from functools import cmp_to_key


def compare(a, b) -> int:
    match a, b:
        case list(), list():
            for c1, c2 in zip(a, b):
                if compare(c1, c2) != 0:
                    return compare(c1, c2)
            return compare(len(a), len(b))
        case list(), int():
            return compare(a, [b])
        case int(), list():
            return compare([a], b)
        case int(), int():
            return a - b
        case _:
            raise ValueError("invalid types")


def part_one(input: str) -> int:
    ans = 0
    for idx, pair in enumerate(input.split("\n\n"), 1):
        left, right = map(literal_eval, pair.split())
        if compare(left, right) < 0:
            ans += idx
    return ans


def part_two(input: str) -> int:
    nodes = list(map(literal_eval, input.strip().split()))
    nodes += [[[2]], [[6]]]
    nodes.sort(key=cmp_to_key(compare))
    ans = 1
    for idx, node in enumerate(nodes, 1):
        if node == [[2]] or node == [[6]]:
            ans *= idx
    return ans


with open("input.txt") as f:
    input = f.read()
    print(f"part one: {part_one(input)}")
    print(f"part two: {part_two(input)}")
