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
fibonacci_iterative10 (5.0s) ...          125.113 ns/iter (1.000 R²)
fibonacci_binets10 (5.0s) ...             61.903 ns/iter (1.000 R²)
fibonacci_iterative100 (5.0s) ...         803.885 ns/iter (1.000 R²)
fibonacci_binets100 (5.0s) ...            61.936 ns/iter (1.000 R²)
fibonacci_iterative1000 (5.0s) ...        7_437.277 ns/iter (1.000 R²)
fibonacci_binets1000 (5.0s) ...           59.601 ns/iter (1.000 R²)
fibonacci_iterative10000 (5.0s) ...       73_576.575 ns/iter (1.000 R²)
fibonacci_binets10000 (5.0s) ...          40.186 ns/iter (1.000 R²)

lucas_iterative10 (5.0s) ...              124.157 ns/iter (1.000 R²)
lucas_binets10 (5.0s) ...                 59.926 ns/iter (1.000 R²)
lucas_iterative100 (5.0s) ...             802.590 ns/iter (1.000 R²)
lucas_binets100 (5.0s) ...                59.922 ns/iter (1.000 R²)
lucas_iterative1000 (5.0s) ...            7_429.068 ns/iter (1.000 R²)
lucas_binets1000 (5.0s) ...               60.413 ns/iter (1.000 R²)
lucas_iterative10000 (5.0s) ...           73_636.230 ns/iter (1.000 R²)
lucas_binets10000 (5.0s) ...              40.119 ns/iter (1.000 R²)

factorial_naive10 (5.0s) ...              50.143 ns/iter (1.000 R²)
factorial_memoized10 (5.0s) ...           17.167 ns/iter (1.000 R²)
factorial_naive100 (5.0s) ...             298.151 ns/iter (1.000 R²)
factorial_memoized100 (5.0s) ...          17.174 ns/iter (1.000 R²)
factorial_naive1000 (5.0s) ...            2_703.372 ns/iter (1.000 R²)
factorial_memoized1000 (5.0s) ...         17.188 ns/iter (1.000 R²)
factorial_naive10000 (5.0s) ...           26_662.082 ns/iter (1.000 R²)
factorial_memoized10000 (5.0s) ...        17.171 ns/iter (1.000 R²)
```
