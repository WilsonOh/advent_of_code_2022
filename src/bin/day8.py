import numpy as np


def main():
    with open("input.txt") as f:
        grid = np.array([list(map(int, list(line.strip()))) for line in f.readlines()])
        n = len(grid)
        m = len(grid[0])

        def part_one():
            ans = 0
            for i in range(1, n - 1):
                for j in range(1, m - 1):
                    curr = grid[i][j]
                    if (
                        np.all([x < curr for x in grid[:, j][0:i]])
                        or np.all([x < curr for x in grid[:, j][i + 1 : n]])
                        or np.all([x < curr for x in grid[i, :][0:j]])
                        or np.all([x < curr for x in grid[i, :][j + 1 : m]])
                    ):
                        ans += 1
            return ans + (2 * m) + (2 * n) - 4

        def count_trees_in_view(grid_slice, curr):
            ans = 0
            for tree in grid_slice:
                if tree < curr:
                    ans += 1
                else:
                    return ans + 1
            return ans

        def part_two():
            ans = 0
            for i in range(1, n - 1):
                for j in range(1, m - 1):
                    curr = grid[i][j]
                    scenic_score = (
                        count_trees_in_view(grid[:, j][0:i][::-1], curr)
                        * count_trees_in_view(grid[:, j][i + 1 : n], curr)
                        * count_trees_in_view(grid[i, :][0:j][::-1], curr)
                        * count_trees_in_view(grid[i, :][j + 1 : m], curr)
                    )
                    ans = max(ans, scenic_score)
            return ans

        print(f"part one: {part_one()}")
        print(f"part two: {part_two()}")


if __name__ == "__main__":
    main()
