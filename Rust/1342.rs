/**
 * 1342. Number of Steps to Reduce a Number to Zero
 * https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
 */

impl Solution {
  pub fn number_of_steps(num: i32) -> i32 {
    let mut num: i32 = num;
    let mut out: i32 = 0;

    while num != 0 {
      if num % 2 == 0 {
        num /= 2;
      } else {
        num -= 1;
      }

      out += 1;
    }

    return out;
  }
}
