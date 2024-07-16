/**
 * 3190. Find Minimum Operations to Make All Elements Divisible by Three
 * https://leetcode.com/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/
 */

impl Solution {
  pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut out: i32 = 0;

    for n in nums {
      out += std::cmp::min(n % 3, 3 - (n % 3));
    }

    return out;
  }
}
