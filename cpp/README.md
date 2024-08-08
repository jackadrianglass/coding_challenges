# Setup

Tools used to build

- C++20 complient compiler
- Conan Version 2.6.0
- Ninja Version 1.12.1
- Meson Version 1.3.0

# Q1

Description: Write an interweave function that operates on tuples. This function will return a
tuple with the elements from the tuples alternated between, used as such:

```c++
std::tuple<int, int, int> a {0, 1, 2};
std::tuple<char, char, char> b {'a', 'b', 'c'};
std::tuple<int, char, int, char, int, char> ret = interweave(a, b);
//returns {0, 'a', 1, 'b', 2, 'c'};
```

● Optional extension: Have this operate on all tuple-like types (e.g., std::pair,
std::array,…). Partial solutions are acceptable for discussion as well.
● Optional extension: Generalize to N-tuples

## Notepad

How do I go about implementing this?
1. Start with some minimal base cases that concat a single element tuple to another
2. Figure out how to iterate over tuples without caring what the return type is
3. Do some `decltype` magic to get the return type all squared away

- [x] Main problem
- [ ] Operate on tuple-like things
- [ ] Operate on N-tuples
- [x] Figure out how to get the unit testing framework to print out the values

# Q2

Description: When using visit on a variant, all of the functions in the visitor are required to
return the same type. Write a map function which takes a visitor with possibly different return
types and returns a variant encapsulating all of the return types.

```c++
template<class... Ts>
struct Visitor : Ts... {
    using Ts::operator()...;
};
auto std::variant<int64_t, double> ret = map(
    Visitor{
        [](int32_t x)->int64_t{ return x*2; },
        [](float x)->double { return x*2.f; }
    },
    std::variant<int, float>{2U}
);
```

## Notepad

I need to get the resulting type of the visitor pattern mapped into the variant
type. Since `std::visit` requires that all return types be the same, I can't use
that.

