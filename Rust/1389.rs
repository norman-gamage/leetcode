/**
 * 1389. Create Target Array in the Given Order
 * https://leetcode.com/problems/create-target-array-in-the-given-order/
 */

impl Solution {
  pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();

    for (i, v) in nums.into_iter().enumerate() {
      out.insert(index[i] as usize, v);
    }

    return out;
  }
}
