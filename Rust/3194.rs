/**
 * 3194. Minimum Average of Smallest and Largest Elements
 * https://leetcode.com/problems/minimum-average-of-smallest-and-largest-elements/
 */

impl Solution {
  pub fn minimum_average(nums: Vec<i32>) -> f64 {
    let mut nums: Vec<i32> = nums;
    nums.sort_unstable();
    let mut out: f64 = f64::MAX;
    let len: usize = nums.len() as usize;

    for i in 0..len / 2 {
      let avg: f64 = ((nums[i] + nums[len - i - 1]) as f64) / 2.0;
      if avg < out {
        out = avg;
      }
    }

    return out;
  }
}
