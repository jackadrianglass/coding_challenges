#![allow(dead_code)]
use microbench;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("client/cxx/calculator.h");

        type Calculator;
        fn new_calculator() -> UniquePtr<Calculator>;

        fn fibonacci_iterative(&self, val: i32) -> i32;
        fn fibonacci_binets(&self, val: i32) -> i32;
        fn golden_ratio(&self, val: i32) -> f64;
        fn lucas_iterative(&self, val: i32) -> i32;
        fn lucas_binets(&self, val: i32) -> i32;
        fn factorial_naive(&self, val: i32) -> i32;
        fn factorial_memoized(&self, val: i32) -> i32;
    }
}

pub fn benchmark() {
    let calculator = ffi::new_calculator();
    let options = microbench::Options::default();
    println!("Golden ratio is {}", calculator.golden_ratio(20));
    microbench::bench(&options, "fibonacci_iterative", || {
        calculator.fibonacci_iterative(10)
    });
    microbench::bench(&options, "fibonacci_binets", || {
        calculator.fibonacci_binets(10)
    });
    microbench::bench(&options, "lucas_iterative", || {
        calculator.lucas_iterative(10)
    });
    microbench::bench(&options, "lucas_binets", || calculator.lucas_binets(10));
    microbench::bench(&options, "factorial_naive", || {
        calculator.factorial_naive(10)
    });
    microbench::bench(&options, "factorial_memoized", || {
        calculator.factorial_memoized(10)
    });
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn fibonacci() {
        let calculator = ffi::new_calculator();
        let values = vec![
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (7, 13),
        ];
        for (input, expected) in values.iter() {
            assert_eq!(calculator.fibonacci_iterative(*input), *expected);
            assert_eq!(calculator.fibonacci_binets(*input), *expected);
        }
    }

    #[test]
    fn lucas() {
        let calculator = ffi::new_calculator();
        let values = vec![
            (0, 2),
            (1, 1),
            (2, 3),
            (3, 4),
            (4, 7),
            (5, 11),
            (6, 18),
            (7, 29),
        ];
        for (input, expected) in values.iter() {
            assert_eq!(calculator.lucas_iterative(*input), *expected);
            assert_eq!(calculator.lucas_binets(*input), *expected);
        }
    }

    #[test]
    fn factorial() {
        let calculator = ffi::new_calculator();
        let values = vec![(0, 1), (1, 1), (2, 2), (3, 6), (4, 24), (5, 120), (6, 720)];

        for (input, expected) in values.iter() {
            assert_eq!(calculator.factorial_naive(*input), *expected);
            assert_eq!(calculator.factorial_memoized(*input), *expected);
        }
    }
}
