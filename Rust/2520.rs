/**
 * 2520. Count the Digits That Divide a Number
 * https://leetcode.com/problems/count-the-digits-that-divide-a-number/
 */

impl Solution {
  pub fn count_digits(num: i32) -> i32 {
    let mut _num = num;
    let mut out = 0;

    while _num > 0 {
      let last_digit = _num % 10;

      if num % last_digit == 0 {
        out += 1;
      }

      _num /= 10;
    }

    return out;
  }
}
