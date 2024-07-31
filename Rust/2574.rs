/**
 * 2574. Left and Right Sum Differences
 * https://leetcode.com/problems/left-and-right-sum-differences/submissions/1339247246/
 */

impl Solution {
  pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let SUM: i32 = nums.iter().sum();
    let mut out: Vec<i32> = vec![0; nums.len()];
    let mut left_sum: i32 = 0;

    for i in 0..nums.len() {
      out[i] = (left_sum - (SUM - nums[i] - left_sum)).abs();
      left_sum += nums[i];
    }

    return out;
  }
}
