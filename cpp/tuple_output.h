#include <iostream>
#include <tuple>
#include <variant>

template <class T, size_t... I>
void print(std::ostream &stream, const T &val, std::index_sequence<I...>) {
  stream << "(";
  (..., (stream << (I == 0 ? "" : ", ") << std::get<I>(val)));
  stream << ")";
}

template <typename... T>
auto operator<<(std::ostream &stream, std::tuple<T...> val) -> std::ostream & {
  print(stream, val, std::make_index_sequence<sizeof...(T)>());
  return stream;
}

// template <typename... T>
// auto operator<<(std::ostream &stream,
//                 std::variant<T...> val) -> std::ostream & {
//   stream << "(";
//   std::visit([&](auto &&arg) { stream << arg; }, val);
//   stream << ")";
//   return stream;
// }
