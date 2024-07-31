/**
 * 1365. How Many Numbers Are Smaller Than the Current Number
 * https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
 */

impl Solution {
  pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    let mut sorted_nums = nums.clone();
    sorted_nums.sort_unstable();

    for v in nums.iter() {
      out.push(
        sorted_nums
          .iter()
          .position(|n| n == v)
          .unwrap() as i32
      );
    }

    return out;
  }
}
