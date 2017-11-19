/* Find the sum of all the numbers from 0 to an upper limit that are divisible
 * by 3 or 5
 * */
#[allow(dead_code)]
pub fn solve(upper: i32) -> i32 {
    let mut sum: i32 = 0;
    for x in 0..upper {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }
    sum
}
