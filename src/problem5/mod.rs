
pub fn solve(n: i64) -> i64 {
    let mut number = 0;
    let mut done = false;

    while !done { 
        number += n; // the number must divisible by the biggest number in the sequence
        let all = (1..n).rev().all(|x| number % x == 0);
        if all {
            done = true;
        }
    }
    number
}
