/**
 * 2000. Reverse Prefix of Word
 * https://leetcode.com/problems/reverse-prefix-of-word/
 */

impl Solution {
  pub fn reverse_prefix(word: String, ch: char) -> String {
    match word.find(ch) {
      None => {
        return word;
      }

      Some(first_ind) => {
        return [
          &word[0..first_ind + 1].chars().rev().collect::<String>(),
          &word[first_ind + 1..],
        ].concat();
      }
    }
  }
}
