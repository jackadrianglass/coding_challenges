#include "calculator.h"
#include <ranges>

auto Calculator::fibonacci(int depth) const -> int {
	if(depth == 0) { return 0; }
	if(depth == 1) { return 1; }
	auto values = std::pair{0, 1};
	for([[maybe_unused]] auto const _ : std::ranges::views::iota(1, depth)){
		auto const [before, last] = values;
		values = std::pair{ last, last + before };
	}
	return values.second;
}
auto Calculator::lucas(int depth) const -> int {
	if(depth == 0) { return 2; }
	if(depth == 1) { return 1; }
	auto values = std::pair{2, 1};
	for([[maybe_unused]] auto const _ : std::ranges::views::iota(1, depth)){
		auto const [before, last] = values;
		values = std::pair{ last, last + before };
	}
	return values.second;
}

auto Calculator::factorial(int depth) const -> int {
	if(depth == 0) { return 1; }
	auto last = 1;
	for([[maybe_unused]] auto const value : std::ranges::views::iota(1, depth + 1)){
		last = last * value;
	}
	return last;
}

[[nodiscard]] auto new_calculator() -> std::unique_ptr<Calculator> {
	return std::make_unique<Calculator>();
}
