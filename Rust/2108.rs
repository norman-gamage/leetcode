/**
 * 2108. Find First Palindromic String in the Array
 * https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
 */

impl Solution {
  pub fn first_palindrome(words: Vec<String>) -> String {
    let mut out: String = "".to_string();

    for w in words {
      let _w: String = w.chars().rev().collect::<String>();

      if w == _w {
        out = w;
        break;
      }
    }

    return out;
  }
}
