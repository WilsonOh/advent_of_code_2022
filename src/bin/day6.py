from window import Window


def main():
    with open("input.txt") as f:
        input = f.read().strip()

        def find_marker(n: int) -> int:
            for i in range(len(input) - n):
                if len(set(input[i : i + n])) == n:
                    return i + n
            return -1

        def find_marker_window(window_size: int) -> int:
            for idx, window in enumerate(Window(input, window_size, strict=True)):
                if len(set(window)) == window_size:
                    return idx + window_size
            return -1

        print(f"part one: {find_marker_window(4)}")
        print(f"part two: {find_marker_window(14)}")


if __name__ == "__main__":
    main()
