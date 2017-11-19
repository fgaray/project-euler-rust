// Misc utils for numbers


/* Reverse a number using only arithmetic operators
 *
 * Complexity:
 *  Time: O(n)
 *  Space: O(1)
 *
 * For n digits
 *
 * */
pub fn reverse(n: i32) -> i32 {
    let mut reversed: i32 = 0;
    let mut number: i32 = n;
    while number > 0 {
        reversed = reversed*10 + number%10;
        number = number / 10;
    }
    reversed
}
