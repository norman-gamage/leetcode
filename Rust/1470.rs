/**
 * 1470. Shuffle the Array
 * https://leetcode.com/problems/shuffle-the-array/
 */

impl Solution {
  pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut out = Vec::with_capacity(nums.len());

    for i in 0..n {
      out.push(nums[i as usize]);
      out.push(nums[(i + n) as usize]);
    }

    return out;
  }
}
