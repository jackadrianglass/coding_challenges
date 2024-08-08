#pragma once
#include <memory>

class Calculator {
public:
	[[nodiscard]] auto fibonacci(int value) const -> int;
	[[nodiscard]] auto lucas(int value) const -> int;
	[[nodiscard]] auto factorial(int value) const -> int;
};

[[nodiscard]] auto new_calculator() -> std::unique_ptr<Calculator>;
