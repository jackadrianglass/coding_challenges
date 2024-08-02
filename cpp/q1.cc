#include "tuple_output.h"

#include <boost/ut.hpp>
#include <ranges>
#include <tuple>
#include <utility>

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

template <size_t idx, typename... T1, typename... T2, typename... T3>
constexpr auto interweave_impl(std::tuple<T1...> const lhs,
                               std::tuple<T2...> const rhs,
                               std::size_t const max) {
  if constexpr (idx >= std::tuple_size<decltype(lhs)>() ||
                idx >= std::tuple_size<decltype(rhs)>()) {
    return std::make_tuple();
  } else {
    return std::tuple_cat(
        std::make_tuple(std::get<idx>(lhs), std::get<idx>(rhs)),
        interweave_impl<idx + 1>(lhs, rhs, max));
  }
}

template <typename... T1, typename... T2>
constexpr auto interweave(std::tuple<T1...> lhs, std::tuple<T2...> rhs) {
  auto const [smaller, bigger] = [=] {
    if constexpr (std::tuple_size<decltype(lhs)>() <
                  std::tuple_size<decltype(rhs)>()) {
      return std::pair{lhs, rhs};
    } else {
      return std::pair{rhs, lhs};
    }
  }();
  constexpr auto end = std::tuple_size<decltype(smaller)>();
  return std::tuple_cat(interweave_impl<0>(lhs, rhs, end),
                        trim_till<end>(bigger));
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
});

int main() {}
