
use utils::prime;
use std;


pub fn solve(n: i64) -> i64 {
   let mut i = 3;
   let mut prime = prime::Prime::new();

   let s = (n as f64).sqrt() as i64 + 1;

   // create all the primes from 3 to sqrt(n)
   while i < s {
       prime.is_prime(i);
       i += 2
   }

   // iterate in revese order, the first prime that can divide the number is the biggest prime
   // factor
   let mut v = prime.get_mem();
   for x in v.iter().rev() {
       if n % x == 0 {
            return *x;
       }
   }

   0
}
