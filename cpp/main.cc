#include <cstdint>
#include <ranges>
#include <tuple>
#include <utility>
#include <variant>

#include "tuple_output.h"

#include <boost/ut.hpp>

////////////////////////////////////////////////////////////////////////////////////////
// Q1
////////////////////////////////////////////////////////////////////////////////////////

template <std::size_t idx, typename... T, std::size_t... I>
constexpr auto trim_till_(std::tuple<T...> const &val,
                          std::index_sequence<I...>) {
  return std::make_tuple(std::get<I + idx>(val)...);
}

/// Trims till the index provided
template <std::size_t idx, typename... T>
constexpr auto trim_till(std::tuple<T...> const &val) {
  return trim_till_<idx>(val, std::make_index_sequence<sizeof...(T) - idx>());
}

template <typename T>
[[nodiscard]]
consteval auto size_of(T const &val) -> std::size_t {
  if constexpr (requires { val.size(); }) {
    return std::size(val);
  } else {
    return std::tuple_size<T>();
  }
}

template <std::size_t idx, typename... Xs>
constexpr auto interweave_row(Xs &&...) {
  return std::tuple{};
}

template <std::size_t idx, typename Head, typename... Xs>
constexpr auto interweave_row(Head &&head, Xs &&...xs) {
  if constexpr (idx < std::tuple_size_v<std::remove_reference_t<Head>>) {
    return std::tuple_cat(std::make_tuple(std::get<idx>(head)),
                          interweave_row<idx>(std::forward<Xs>(xs)...));
  } else {
    return interweave_row<idx>(std::forward<Xs>(xs)...);
  }
}
template <std::size_t idx, typename... Xs>
constexpr auto interweave_impl(Xs &&...) {
  return std::tuple{};
}

template <std::size_t idx, typename Head, typename... Xs>
constexpr auto interweave_impl(Head &&head, Xs &&...xs) {
  if constexpr (idx < std::tuple_size_v<std::remove_reference_t<Head>>) {
    return std::tuple_cat(
        interweave_row<idx>(std::forward<Head>(head), std::forward<Xs>(xs)...),
        interweave_impl<idx + 1>(std::forward<Head>(head),
                                 std::forward<Xs>(xs)...));
  } else {
    return interweave_impl<idx>(std::forward<Xs>(xs)...);
  }
}

template <typename... T> constexpr auto interweave(T &&...xs) {
  return interweave_impl<0>(std::forward<T>(xs)...);
}

using namespace boost::ut;

auto const _1 = suite<"trim tests">([] {
  "can trim nothing"_test = [] {
    auto const val = std::make_tuple(1, 2, 3, 4);
    expect(that % trim_till<0>(val) == val);
  };
  "can trim a tuple"_test = [] {
    expect(that % trim_till<1>(std::make_tuple(1, 2, 3, 4)) ==
           std::make_tuple(2, 3, 4));
  };
  "won't trim past end"_test = [] {
    expect(that % trim_till<4>(std::make_tuple(1, 2, 3, 4)) ==
           std::make_tuple());
  };
});

auto const _2 = suite<"interweave tests">([] {
  "both empty"_test = [] {
    expect(that % interweave(std::make_tuple(), std::make_tuple()) ==
           std::make_tuple());
  };
  "one has elements"_test = [] {
    expect(that % interweave(std::make_tuple(1, 2, 3), std::make_tuple()) ==
           std::make_tuple(1, 2, 3));
    expect(that % interweave(std::make_tuple(), std::make_tuple(1, 2, 3)) ==
           std::make_tuple(1, 2, 3));
  };

  "same type, same size"_test = [] {
    expect(that %
               interweave(std::make_tuple(1, 2, 3), std::make_tuple(4, 5, 6)) ==
           std::make_tuple(1, 4, 2, 5, 3, 6));
  };
  "different types, same size"_test = [] {
    expect(that % interweave(std::make_tuple(1, 'a', std::string("banana")),
                             std::make_tuple(4.0, 5.0f, 5)) ==
           std::make_tuple(1, 4.0, 'a', 5.0f, std::string("banana"), 5));
  };
  "same type, different sizes"_test = [] {
    expect(that % interweave(std::make_tuple(1, 2, 3), std::make_tuple(4, 5)) ==
           std::make_tuple(1, 4, 2, 5, 3));
    expect(that % interweave(std::make_tuple(1, 2), std::make_tuple(3, 4, 5)) ==
           std::make_tuple(1, 3, 2, 4, 5));
  };
  "challenges's example"_test = [] {
    auto const a = std::tuple{0, 1, 2};
    auto const b = std::tuple{'a', 'b', 'c'};
    auto const ret = std::tuple{0, 'a', 1, 'b', 2, 'c'};
    expect(that % interweave(a, b) == ret);
  };
  "can interweave many tuples"_test = [] {
    expect(that % interweave(std::make_tuple(1, 2, 3), std::make_tuple(4, 5),
                             std::make_tuple(6, 7, 8)) ==
           std::make_tuple(1, 4, 6, 2, 5, 7, 3, 8));
  };
});

////////////////////////////////////////////////////////////////////////////////////////
// Q2
////////////////////////////////////////////////////////////////////////////////////////

template <class... Ts> struct Visitor : Ts... {
  using Ts::operator()...;
};

template <typename Visitor, typename... T>
constexpr auto map(Visitor visitor, std::variant<T...> val)
    -> std::variant<std::invoke_result_t<Visitor, T>...> {
  return std::visit(
      [&](auto &&arg) -> std::variant<std::invoke_result_t<Visitor, T>...> {
        return visitor(std::forward<decltype(arg)>(arg));
      },
      val);
}

auto const _3 = suite<"visitor map tests">([] {
  "challenges's example"_test = [] {
    auto const result = map(Visitor{[](int32_t x) -> int64_t { return x * 2; },
                                    [](float x) -> double { return x * 2.f; }},
                            std::variant<int32_t, float>{2});
    auto const expected = std::variant<int64_t, double>{4};

    static_assert(std::is_same<decltype(result), decltype(expected)>());
    expect(expected == result);
  };
});

int main() {}
