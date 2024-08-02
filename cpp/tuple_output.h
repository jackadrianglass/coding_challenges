#include <iostream>
#include <tuple>

template <class T, size_t... I>
void print(std::ostream &stream, const T &val, std::index_sequence<I...>) {
  stream << "(";
  (..., (stream << (I == 0 ? "" : ", ") << std::get<I>(val)));
  stream << ")";
}

template <typename... T>
std::ostream &operator<<(std::ostream &stream, std::tuple<T...> val) {
  print(stream, val, std::make_index_sequence<sizeof...(T)>());
  return stream;
}
