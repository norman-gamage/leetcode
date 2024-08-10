/**
 * 1313. Decompress Run-Length Encoded List
 * https://leetcode.com/problems/decompress-run-length-encoded-list/
 */

impl Solution {
  pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();

    for i in (0..nums.len()).step_by(2) {
      for j in 0..nums[i] {
        out.push(nums[i + 1]);
      }
    }

    return out;
  }
}
