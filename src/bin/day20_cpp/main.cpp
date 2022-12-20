#include <cmath>
#include <cstdio>
#include <fmt/core.h>
#include <fmt/ranges.h>
#include <iostream>
#include <list>
#include <vector>

int rem(const int a, const int b) { return ((a % b) + b) % b; }

auto move_it(std::list<int> &nums, auto it, int num_steps) {
  if (num_steps == 0) { return it; }
  int abs_step = std::abs(num_steps);
  if (num_steps < 0) {
    while (abs_step--) {
      if (it == nums.begin()) {
        it = --nums.end();
      } else {
        --it;
      }
    }
  } else {
    while (abs_step--) {
      if (it == nums.end()) {
        it = nums.begin();
      } else {
        ++it;
      }
    }
  }
  return it;
}

int main() {
  std::freopen("sample.txt", "r", stdin);
  std::ios_base::sync_with_stdio(false);
  std::cin.tie(nullptr);
  std::list<int> nums;
  std::vector<std::list<int>::iterator> its;
  int curr;
  while (std::cin >> curr) { nums.push_back(curr); }
  for (auto it = nums.begin(); it != nums.end(); ++it) { its.push_back(it); }
  for (const auto &it : its) {
    int shift_num = *it;
    auto new_pos = move_it(nums, it, shift_num);
    nums.erase(it);
    nums.insert(new_pos, shift_num);
  }
  fmt::print("{}", nums);
}
