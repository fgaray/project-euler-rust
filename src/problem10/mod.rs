
use utils::prime;
use std;


pub fn solve(n: i64) -> i64 {
   let mut prime = prime::Prime::new();
   let mut sum = 2;
   let mut x = 3;

   while x < n {
        if prime.is_prime(x) {
            sum += x;
        }
        x += 2;
   }

   sum
}
