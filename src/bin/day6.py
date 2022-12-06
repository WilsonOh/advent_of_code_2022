def main():
    with open("input.txt") as f:
        input = f.read().strip()

        def find_marker(n: int) -> int:
            for i in range(len(input) - n):
                if len(set(input[i : i + n])) == n:
                    return i + n
            return -1

        print(f"part one: {find_marker(4)}")
        print(f"part two: {find_marker(14)}")


if __name__ == "__main__":
    main()
