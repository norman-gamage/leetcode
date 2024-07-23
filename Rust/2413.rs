/**
 * 2413. Smallest Even Multiple
 * https://leetcode.com/problems/smallest-even-multiple/
 */

impl Solution {
  pub fn smallest_even_multiple(n: i32) -> i32 {
    return if n % 2 == 0 { n } else { n * 2 };
  }
}
