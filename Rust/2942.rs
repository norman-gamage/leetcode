/**
 * 2942. Find Words Containing Character
 * https://leetcode.com/problems/find-words-containing-character/
 */

impl Solution {
  pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();

    for (i, w) in words.iter().enumerate() {
      if w.find(x).is_some() {
        out.push(i as i32);
      }
    }

    return out;
  }
}
