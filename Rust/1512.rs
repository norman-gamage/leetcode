/**
 * 1512. Number of Good Pairs
 * https://leetcode.com/problems/number-of-good-pairs/
 */

use std::collections::HashMap;

impl Solution {
  pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut num_count: HashMap<i32, i32> = HashMap::new();
    let mut out: i32 = 0;

    for n in nums {
      num_count
        .entry(n)
        .and_modify(|n| {
          *n += 1;
        })
        .or_insert(1);
    }

    for (_, v) in num_count {
      out += if v > 1 { (v * (v - 1)) / 2 } else { 0 };
    }

    return out;
  }
}
