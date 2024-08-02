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
1. Compile it into a shared library.
1. Use Rust's FFI to call the C++ class methods.
1. Ensure the Rust code handles the C++ class interface correctly.
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
1. Implement the Trait for Different Graph Types:
   ○ Implement the trait for directed and undirected graphs.
1. Provide Methods for Graph Operations:
   ○ Implement methods for adding nodes, adding edges, changing values stored at
   nodes and edges.
   ○ Iterator for traversing the graph.
   ○ The graph can be cloned/copied (efficiently).
   ○ Orientation of edges can be flipped.
1. Ensure Type Safety and Flexibility:
   ○ Use Rust's type system to ensure the graph operations are type-safe and
   flexible.
   Bonus:
1. Implement a graph data structure by allowing cycles.
1. Implement an algorithm (of your choice) on this graph data structure.
1. Add delete for the data OR (not XOR) come prepared to discuss considerations around
   the solution.

Q5)
Description: We quite liked your fractal example in your repository. Currently, you are rendering
2D fractals. Please, either: take this as an opportunity to extend this model to 3D and choose a
3D curve/fractal of your choice to present, OR come prepared to discuss such a project.
