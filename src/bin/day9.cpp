#include <cmath>
#include <fmt/core.h>
#include <fstream>
#include <iostream>
#include <ostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <unordered_set>
#include <vector>

struct Coords {
  int row{};
  int col{};

  Coords() = default;

  Coords(int _row, int _col) : row(_row), col(_col) {}

  bool operator==(const Coords &other) const { return row == other.row && col == other.col; }

  Coords operator+(const Coords &other) const { return { row + other.row, col + other.col }; }

  Coords operator-(const Coords &other) const { return { row - other.row, col - other.col }; }

  Coords &operator+=(const Coords &other) {
    row += other.row;
    col += other.col;
    return *this;
  }

  static Coords normalized(const Coords &coord) {
    Coords other = coord;
    if (other.row != 0) { other.row = (other.row / std::abs(other.row)); }
    if (other.col != 0) { other.col = (other.col / std::abs(other.col)); }
    return other;
  }

  friend std::ostream &operator<<(std::ostream &out, const Coords &other) {
    out << fmt::format("Coords{{ row: {}, col: {} }}", other.row, other.col);
    return out;
  }
};

struct CoordHash {
  std::size_t operator()(const Coords &coord) const {
    return std::hash<int>()(coord.row) ^ std::hash<int>()(coord.col);
  }
};

struct Move {
  Coords coord;
  int mag;
};

Coords adjust_knot(const Coords &head, const Coords &tail) {
  const double dist = std::sqrt(std::pow(head.row - tail.row, 2) + std::pow(head.col - tail.col, 2));
  if (dist < 2.0) { return tail; }
  return tail + Coords::normalized(head - tail);
}

std::size_t knots(const std::vector<Move> &moves, std::size_t num_knots) {
  std::vector<Coords> tails(num_knots);
  std::unordered_set<Coords, CoordHash> visited;
  visited.insert(tails[0]);
  for (const auto &[offset, mag] : moves) {
    for (int _ = 0; _ < mag; ++_) {
      tails[0] += offset;
      for (std::size_t i = 1; i < num_knots; ++i) {
        tails[i] = adjust_knot(tails[i - 1], tails[i]);
        if (i == num_knots - 1) { visited.insert(tails[i]); }
      }
    }
  }
  return visited.size();
}

std::vector<Move> parse_input(const std::string &file_path) {
  std::ifstream ifs(file_path);
  std::string line;
  std::vector<Move> moves;
  while (std::getline(ifs, line)) {
    std::stringstream ss(line);
    char dir;
    int mag;
    ss >> dir >> mag;
    switch (dir) {
    case 'R':
      moves.push_back({ { 0, 1 }, mag });
      break;
    case 'L':
      moves.push_back({ { 0, -1 }, mag });
      break;
    case 'U':
      moves.push_back({ { 1, 0 }, mag });
      break;
    case 'D':
      moves.push_back({ { -1, 0 }, mag });
      break;
    default:
      throw std::runtime_error("invalid direction");
    }
  }
  return moves;
}

int main() {
  std::vector<Move> moves = parse_input("input.txt");
  auto part_one = knots(moves, 2);
  auto part_two = knots(moves, 10);
  fmt::print("part one: {}\n", part_one);
  fmt::print("part two: {}", part_two);
}
