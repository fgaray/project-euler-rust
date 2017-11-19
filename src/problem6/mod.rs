

pub fn solve(upper: i64) -> i64 {
    let sum_square: i64 = (1..(upper + 1)).map(|x| x*x).sum();
    let sum: i64 = (1..(upper + 1)).sum();
    let square_sum = sum*sum;
    square_sum - sum_square
}
