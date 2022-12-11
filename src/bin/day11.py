from math import lcm
from dataclasses import dataclass
from typing import Callable, Deque
from collections import deque
from functools import reduce


@dataclass
class Monkey:
    items: Deque[int]
    operation: Callable[[int], int]
    test: Callable[[int], int]
    time_inspected: int = 0


def get_op_func(line: str) -> Callable[[int], int]:
    match line.lstrip("  Operation: new = old ").split():
        case ["*", "old"]:
            return lambda x: x * x
        case ["*", val]:
            return lambda x: x * int(val)
        case ["+", val]:
            return lambda x: x + int(val)
        case _:
            raise ValueError("invalid operation")


def get_test_func(div_by: int, if_true: int, if_false: int) -> Callable[[int], int]:
    def inner(x: int) -> int:
        if x % div_by == 0:
            return if_true
        return if_false

    return inner


def parse_monkeys(input: str) -> tuple[list[Monkey], int]:
    monkeys = input.split("\n\n")
    ret = []
    mod_by = 1
    for monkey in monkeys:
        monkey = monkey.splitlines()
        items = deque(map(int, monkey[1].lstrip("  Starting items: ").split(", ")))
        op_func = get_op_func(monkey[2])
        div_by = int(monkey[3].lstrip("  Test: divisible by "))
        if_true = int(monkey[4].lstrip("    If true: throw to monkey "))
        if_false = int(monkey[5].lstrip("    If false: throw to monkey "))
        mod_by = lcm(mod_by, div_by)
        ret.append(
            Monkey(
                items=items,
                operation=op_func,
                test=get_test_func(div_by, if_true, if_false),
            )
        )
    return ret, mod_by


def calc_monkey_business(input: str, num_rounds: int, *, div_by: int = 1):
    monkeys, mod_by = parse_monkeys(input)
    for _ in range(num_rounds):
        for monkey in monkeys:
            while len(monkey.items) != 0:
                worry = (monkey.operation(monkey.items.popleft()) // div_by) % mod_by
                monkeys[monkey.test(worry)].items.append(worry)
                monkey.time_inspected += 1
    monkeys.sort(key=lambda x: x.time_inspected)
    return reduce(lambda acc, curr: acc * curr.time_inspected, monkeys[-2:], 1)


def main():
    with open("input.txt") as f:
        input = f.read()
        part_one = calc_monkey_business(input, 20, div_by=3)
        part_two = calc_monkey_business(input, 10_000)
        print(f"part one: {part_one}")
        print(f"part two: {part_two}")


if __name__ == "__main__":
    main()
