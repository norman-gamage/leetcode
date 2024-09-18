/**
 * 2956. Find Common Elements Between Two Arrays
 * https://leetcode.com/problems/find-common-elements-between-two-arrays/
 */

impl Solution {
  pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let ans1: i32 = nums1
      .iter()
      .filter(|&n| nums2.contains(n))
      .count() as i32;

    let ans2: i32 = nums2
      .iter()
      .filter(|&n| nums1.contains(n))
      .count() as i32;

    return [ans1, ans2].to_vec();
  }
}
