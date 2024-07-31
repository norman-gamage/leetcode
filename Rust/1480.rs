/**
 * 1480. Running Sum of 1d Array
 * https://leetcode.com/problems/running-sum-of-1d-array/
 */

impl Solution {
  pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;

    for i in 1..nums.len() {
      nums[i] += nums[i - 1];
    }

    return nums;
  }
}
