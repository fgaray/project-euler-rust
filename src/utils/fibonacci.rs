
pub struct Fibonacci {
    mem: Vec<i32>
}

/* Fibonacci sequence using dynamic programming with a vector.
 *
 * Starts with 1, that is, the number of the index 0 of the sequence is 1 and in
 * the index 1 is 2
 * 
 * 1, 2, 3, 5, 8, 13, 21, 34, 55, 89
 *
 * Amortized O(1), worst case O(n) and space O(n)
 *
 * */
impl Fibonacci {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        Fibonacci { mem: v }
    }

    #[allow(dead_code)]
    pub fn new_with_capacity(capacity: usize) -> Self {
        let mut v = Vec::with_capacity(capacity);
        v.push(1);
        v.push(2);
        Fibonacci { mem: v }
    }

    #[allow(dead_code)]
    pub fn fib(&mut self, n: i32) -> i32 {
        if self.mem.len() > n as usize {
            self.mem[n as usize]
        } else {
            let f = self.fib(n - 1) + self.fib(n - 2);
            self.mem.insert(n as usize, f);
            f
        }
    }
}



/* Simple fibonacci.
 *
 * Given an n index in the fibonacci sequence, returns the corresponding number 
 * in the sequence
 *
 * Complexity:
 *      Run: O(n)
 *      Space: O(1)
 *
 */
#[allow(dead_code)]
pub fn fibonacci_simple(n: i32) -> i32 {
    let mut x: i32 = 1;
    let mut y: i32 = 1;

    for _ in 0..n {
        let aux = y;
        y = x + y;
        x = aux;
    }

    y
}
