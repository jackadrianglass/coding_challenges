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
