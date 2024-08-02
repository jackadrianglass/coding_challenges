Q1)
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

C/C++
Q2)
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

Q3)
Description: Use FFI to call a C++ library from Rust. Please, create a simple C++ class with
methods and call these methods from Rust.
Skills:
FFI, unsafe Rust, linking with C++ libraries, C++ name mangling.
Steps:
1. Write a simple C++ class/system that calculates
(Please, take efficiency into consideration)
a. Fibonacci numbers
b. Lucas numbers
c. Factorials
2. Compile it into a shared library.
3. Use Rust's FFI to call the C++ class methods.
4. Ensure the Rust code handles the C++ class interface correctly.
Bonus: Add an end-point to calculate the digits of the Golden Ratio in terms of the Fibonacci
sequence. Efficiently caching prior values may be helpful.

Q4)
Description: Implement a generic directed tree data structure in Rust that supports storing and
mutating arbitrary data at the nodes and on the edges. This challenge will involve defining traits
with associated types, implementing these traits for different graph types, and using Rust's
generics to create a flexible and reusable data structure.
Skills: Generics, associated types, traits, Rust's type system.
Steps:
1. Define the Graph Trait:
○ Create a trait for graph operations with associated types for nodes and edges.
2. Implement the Trait for Different Graph Types:
○ Implement the trait for directed and undirected graphs.
3. Provide Methods for Graph Operations:
○ Implement methods for adding nodes, adding edges, changing values stored at
nodes and edges.
○ Iterator for traversing the graph.
○ The graph can be cloned/copied (efficiently).
○ Orientation of edges can be flipped.
4. Ensure Type Safety and Flexibility:
○ Use Rust's type system to ensure the graph operations are type-safe and
flexible.
Bonus:
1. Implement a graph data structure by allowing cycles.
2. Implement an algorithm (of your choice) on this graph data structure.
3. Add delete for the data OR (not XOR) come prepared to discuss considerations around
the solution.

Q5)
Description: We quite liked your fractal example in your repository. Currently, you are rendering
2D fractals. Please, either: take this as an opportunity to extend this model to 3D and choose a
3D curve/fractal of your choice to present, OR come prepared to discuss such a project.
