/**
 * 2485. Find the Pivot Integer
 * https://leetcode.com/problems/find-the-pivot-integer/
 */

impl Solution {
  pub fn pivot_integer(n: i32) -> i32 {
    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;
    let mut out: i32 = -1;

    for i in 1..=n {
      sum1 += i;
    }

    for i in 1..=n {
      sum2 += i;

      if sum1 == sum2 {
        out = i;
        break;
      }

      sum1 -= i;
    }

    return out;
  }
}
