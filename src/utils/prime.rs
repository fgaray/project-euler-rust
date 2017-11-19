/**Functions related to prime numbers*/


pub struct Prime {
    mem: Vec<i64>
}


/*
 * Simple prime checker creating a sieve of prime numbers.
 *
 * is_prime assumes that the numbers that are given are in sequence order, that
 * is, is_prime is called like:
 *      is_prime(3), is_prime(4), is_prime(5)
 *
 * In order to create the sieve.
 *
 * */
impl Prime {

    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut v = Vec::new();
        Prime { mem: v }
    }

    #[allow(dead_code)]
    pub fn new_with_capacity(capacity: usize) -> Self {
        let mut v = Vec::with_capacity(capacity);
        Prime { mem: v }
    }


    pub fn is_prime(&mut self, n: i64) -> bool {
        if n % 2 == 0 {
            return false;
        }

        let s = (n as f64).sqrt() as i64 + 1;

        for x in &self.mem {
            if x > &s {
                break
            }

            if n % x == 0 {
                return false;
            }
        }
        self.mem.push(n);
        true
    }

    pub fn get_mem(&mut self) -> &Vec<i64> {
        &self.mem
    }

}





/*
 * Simple way to check if a number is prime. We iterate over all odd numbers
 * until sqrt(n)
 * */
#[allow(dead_code)]
pub fn is_prime(n: i64) -> bool {
    if n % 2 == 0 {
        false
    } else {
        let mut i = 3;
        let s = (n as f64).sqrt() as i64 + 1;
        while i < s {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}
