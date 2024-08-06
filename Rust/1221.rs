/**
 * 1221. Split a String in Balanced Strings
 * https://leetcode.com/problems/split-a-string-in-balanced-strings/
 */

impl Solution {
  pub fn balanced_string_split(s: String) -> i32 {
    let mut out: i32 = 0;
    let mut sum: i32 = 0;

    for c in s.chars() {
      sum += if c == 'R' { 1 } else { -1 };

      if sum == 0 {
        out += 1;
      }
    }

    return out;
  }
}
