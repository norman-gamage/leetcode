/**
 * 3289. The Two Sneaky Numbers of Digitville
 * https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville/
 */

impl Solution {
  pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut cnt: Vec<i32> = vec![0; nums.len()];
    let mut out: Vec<i32> = Vec::new();

    for n in nums {
      cnt[n as usize] += 1;

      if cnt[n as usize] == 2 {
        out.push(n);
      }
    }

    return out;
  }
}
