/**
 * 3146. Permutation Difference between Two Strings
 * https://leetcode.com/problems/permutation-difference-between-two-strings/
 */

use std::collections::HashMap;

impl Solution {
  pub fn find_permutation_difference(s: String, t: String) -> i32 {
    let mut hash_map: HashMap<char, i32> = HashMap::new();
    let mut out: i32 = 0;

    for (i, c) in s.chars().enumerate() {
      hash_map.entry(c).or_insert(i as i32);
    }

    for (i, c) in t.chars().enumerate() {
      out += (hash_map.get(&c).unwrap() - (i as i32)).abs();
    }

    return out;
  }
}
