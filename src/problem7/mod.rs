
use utils::prime;

pub fn solve(n: i64) -> i64 {
    let mut i = 3;
    let mut prime_n = 1;
    let mut prime = prime::Prime::new();
    while prime_n <= n {
        if prime.is_prime(i) {
            prime_n += 1;
            if prime_n == n {
                return i;
            }
        }
        i += 2;
    }
    return 0;
}
