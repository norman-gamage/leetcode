/**
 * 2418. Sort the People
 * https://leetcode.com/problems/sort-the-people/
 */

use std::iter::zip;

impl Solution {
  pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut zipped: Vec<(i32, String)> = zip(heights, names).collect();
    zipped.sort_unstable_by(|a, b| b.cmp(a));
    return zipped
      .into_iter()
      .map(|(_, n)| n)
      .collect::<Vec<String>>();
  }
}
