/**
 * 2810. Faulty Keyboard
 * https://leetcode.com/problems/faulty-keyboard/
 */

impl Solution {
  pub fn final_string(s: String) -> String {
    let mut out: String = String::new();

    for c in s.chars() {
      if c != 'i' {
        out.push(c);
      } else {
        out = out.chars().rev().collect::<String>();
      }
    }

    return out;
  }
}
