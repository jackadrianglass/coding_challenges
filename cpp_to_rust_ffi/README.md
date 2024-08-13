Q3)
Description: Use FFI to call a C++ library from Rust. Please, create a simple C++ class with
methods and call these methods from Rust.

Skills: FFI, unsafe Rust, linking with C++ libraries, C++ name mangling.

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

Output when run on a Macbook Air M2
```
Golden ratio is 1.6180339985218033
fibonacci_iterative (5.0s) ...           122.573 ns/iter (1.000 R²)
fibonacci_binets (5.0s) ...               60.717 ns/iter (1.000 R²)
lucas_iterative (5.0s) ...               121.415 ns/iter (1.000 R²)
lucas_binets (5.0s) ...                   59.025 ns/iter (1.000 R²)
factorial_naive (5.0s) ...                50.332 ns/iter (1.000 R²)
factorial_memoized (5.0s) ...             18.142 ns/iter (1.000 R²)
```
