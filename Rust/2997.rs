/**
 * 2997. Minimum Number of Operations to Make Array XOR Equal to K
 * https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k/
 */

impl Solution {
  pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    return nums
      .into_iter()
      .fold(k, |a, b| a ^ b)
      .count_ones() as i32;
  }
}
