#pragma once
#include <memory>
#include <vector>

class Calculator {
public:
  Calculator();
  [[nodiscard]] auto fibonacci_iterative(int value) const -> int;
  [[nodiscard]] auto fibonacci_binets(int value) const -> int;
  [[nodiscard]] auto lucas_iterative(int value) const -> int;
  [[nodiscard]] auto lucas_binets(int value) const -> int;
  [[nodiscard]] auto factorial_naive(int value) const -> int;
  [[nodiscard]] auto factorial_memoized(int value) const -> int;
  [[nodiscard]] auto golden_ratio(int value) const -> double;

private:
  mutable std::vector<int> precomputed_factorials;
};

[[nodiscard]] auto new_calculator() -> std::unique_ptr<Calculator>;
