/**
 * 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers
 * https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/
 */

use std::cmp;

impl Solution {
  pub fn min_partitions(n: String) -> i32 {
    let mut out: i32 = 0;

    for c in n.chars() {
      out = cmp::max(out, c.to_digit(10).unwrap() as i32);
    }

    return out;
  }
}
