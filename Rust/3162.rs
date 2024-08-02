/**
 * 3162. Find the Number of Good Pairs I
 * https://leetcode.com/problems/find-the-number-of-good-pairs-i/
 */

impl Solution {
  pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let mut out: i32 = 0;
    let nums2: Vec<i32> = nums2
      .into_iter()
      .map(|n| k * n)
      .collect();

    for i in nums2 {
      if i == 1 {
        out += nums1.len() as i32;
        continue;
      }

      for j in &nums1 {
        if i > *j {
          continue;
        }

        if *j % i == 0 {
          out += 1;
        }
      }
    }

    return out;
  }
}
