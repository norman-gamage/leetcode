/**
 * 2859. Sum of Values at Indices With K Set Bits
 * https://leetcode.com/problems/sum-of-values-at-indices-with-k-set-bits/
 */

impl Solution {
  pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    let mut out: i32 = 0;

    for (i, v) in nums.into_iter().enumerate() {
      if (i.count_ones() as i32) == k {
        out += v;
      }
    }

    return out;
  }
}
