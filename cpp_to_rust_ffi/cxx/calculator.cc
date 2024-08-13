#include "calculator.h"
#include <iostream>
#include <ranges>

namespace views = std::ranges::views;

Calculator::Calculator() {
  // memoize the factorial calculation
  precomputed_factorials.push_back(1);
}

auto Calculator::fibonacci_iterative(int depth) const -> int {
  if (depth == 0) {
    return 0;
  }
  if (depth == 1) {
    return 1;
  }
  auto values = std::pair{0, 1};
  for ([[maybe_unused]] auto const _ : views::iota(1, depth)) {
    auto const [before, last] = values;
    values = std::pair{last, last + before};
  }
  return values.second;
}

auto Calculator::lucas_iterative(int depth) const -> int {
  if (depth == 0) {
    return 2;
  }
  if (depth == 1) {
    return 1;
  }
  auto values = std::pair{2, 1};
  for ([[maybe_unused]] auto const _ : views::iota(1, depth)) {
    auto const [before, last] = values;
    values = std::pair{last, last + before};
  }
  return values.second;
}

auto Calculator::fibonacci_binets(int depth) const -> int {
  auto const a = std::pow(1 + std::sqrt(5), depth);
  auto const b = std::pow(1 - std::sqrt(5), depth);
  auto const c = std::pow(2, depth) * std::sqrt(5);
  return static_cast<int>((a - b) / c);
}

auto Calculator::lucas_binets(int depth) const -> int {
  auto const a = std::pow((1 + std::sqrt(5)) / 2.0, depth);
  auto const b = std::pow((1 - std::sqrt(5)) / 2.0, depth);
  return static_cast<int>(a + b);
}

auto Calculator::golden_ratio(int value) const -> double {
  return static_cast<double>(fibonacci_binets(value + 1)) /
         static_cast<double>(fibonacci_binets(value));
}

auto Calculator::factorial_naive(int depth) const -> int {
  if (depth == 0) {
    return 1;
  }

  auto last = 1;
  for ([[maybe_unused]] auto const value : views::iota(1, depth + 1)) {
    last = last * value;
  }
  return last;
}

auto Calculator::factorial_memoized(int depth) const -> int {
  if (depth == 0) {
    return 1;
  } else if (depth < static_cast<int>(precomputed_factorials.size())) {
    return precomputed_factorials[depth];
  }

  auto last = *precomputed_factorials.rbegin();
  for ([[maybe_unused]] auto const value : views::iota(
           static_cast<int>(precomputed_factorials.size()), depth + 1)) {
    last = last * value;
    precomputed_factorials.push_back(last);
  }
  return last;
}

[[nodiscard]] auto new_calculator() -> std::unique_ptr<Calculator> {
  return std::make_unique<Calculator>();
}
