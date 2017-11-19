

// if we solve the equation we only need to iterate b and c, a is defined as a
// function of b and c
pub fn solve(constant: i64) -> i64 {
    for b in (1..1000) {
        for c in (1..1000) {
            if (constant - b - c).pow(2) + b.pow(2) - c.pow(2) == 0 {
                return (((c.pow(2) - b.pow(2)) as f64).sqrt() as i64)*b*c
            }
        }
    }
    return 0
}
