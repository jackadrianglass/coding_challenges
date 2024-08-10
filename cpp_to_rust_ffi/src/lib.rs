#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("client/cxx/calculator.h");

        type Calculator;
        fn new_calculator() -> UniquePtr<Calculator>;
        fn fibonacci(&self, val: i32) -> i32;
        fn lucas(&self, val: i32) -> i32;
        fn factorial(&self, val: i32) -> i32;
    }
}

pub fn do_the_thing() {
    let calculator = ffi::new_calculator();
    println!("Hello fib {}", calculator.fibonacci(3));
    println!("Hello lucas {}", calculator.lucas(3));
    println!("Hello factorial {}", calculator.factorial(3));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn fibonacci() {
        let calculator = ffi::new_calculator();
        assert_eq!(calculator.fibonacci(0), 0);
        assert_eq!(calculator.fibonacci(1), 1);
        assert_eq!(calculator.fibonacci(2), 1);
        assert_eq!(calculator.fibonacci(3), 2);
        assert_eq!(calculator.fibonacci(4), 3);
        assert_eq!(calculator.fibonacci(5), 5);
        assert_eq!(calculator.fibonacci(6), 8);
        assert_eq!(calculator.fibonacci(7), 13);
    }

    #[test]
    fn lucas() {
        let calculator = ffi::new_calculator();
        assert_eq!(calculator.lucas(0), 2);
        assert_eq!(calculator.lucas(1), 1);
        assert_eq!(calculator.lucas(2), 3);
        assert_eq!(calculator.lucas(3), 4);
        assert_eq!(calculator.lucas(4), 7);
        assert_eq!(calculator.lucas(5), 11);
        assert_eq!(calculator.lucas(6), 18);
        assert_eq!(calculator.lucas(7), 29);
    }

    #[test]
    fn factorial() {
        let calculator = ffi::new_calculator();
        assert_eq!(calculator.factorial(0), 1);
        assert_eq!(calculator.factorial(1), 1);
        assert_eq!(calculator.factorial(2), 2);
        assert_eq!(calculator.factorial(3), 6);
        assert_eq!(calculator.factorial(4), 24);
        assert_eq!(calculator.factorial(5), 120);
        assert_eq!(calculator.factorial(6), 720);
    }
}
