
use utils::fibonacci;

#[allow(dead_code)]
pub fn solve(upper: i32) -> i32 {
    let mut fib = fibonacci::Fibonacci::new_with_capacity(upper as usize);
    let mut n = 0;
    let mut sum = 0;

    while fib.fib(n) < upper {
        let f = fib.fib(n);
        if f % 2 == 0 {
            sum += f;
        }
        n += 1;
    }

    sum
}
