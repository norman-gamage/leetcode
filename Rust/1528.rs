/**
 * 1528. Shuffle String
 * https://leetcode.com/problems/shuffle-string/
 */

impl Solution {
  pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let s: Vec<char> = s.chars().collect();
    let mut out: Vec<char> = vec![' '; indices.len()];

    for (i, v) in indices.iter().enumerate() {
      out[*v as usize] = s[i];
    }

    return out.iter().collect();
  }
}
