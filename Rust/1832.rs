/**
 * 1832. Check if the Sentence Is Pangram
 * https://leetcode.com/problems/check-if-the-sentence-is-pangram/
 */

impl Solution {
  pub fn check_if_pangram(sentence: String) -> bool {
    if sentence.len() < 26 {
      return false;
    }

    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
      match sentence.find(c) {
        Some(_) => {
          continue;
        }
        None => {
          return false;
        }
      }
    }

    return true;
  }
}
