/**
 * 2824. Count Pairs Whose Sum is Less than Target
 * https://leetcode.com/problems/count-pairs-whose-sum-is-less-than-target/
 */

impl Solution {
  pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let mut out: i32 = 0;

    for i in 0..nums.len() - 1 {
      for j in i + 1..nums.len() {
        if nums[i] + nums[j] < target {
          out += 1;
        }
      }
    }

    return out;
  }
}
