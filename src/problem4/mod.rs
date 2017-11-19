
use utils::number;
use std;

pub fn solve() -> i32 {
    let mut d1 = 1;
    let mut d2 = 1;
    let mut largest = 0;

    for d1 in 100..1000 {
        for d2 in 100..1000 {
            let product = d1 * d2;
            if number::reverse(product) == product {
                largest = std::cmp::max(largest, product);
            }
        }
    }
    largest
}
