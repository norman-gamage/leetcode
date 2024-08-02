/**
 * 2114. Maximum Number of Words Found in Sentences
 * https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
 */

impl Solution {
  pub fn most_words_found(sentences: Vec<String>) -> i32 {
    return sentences
      .into_iter()
      .map(|s| s.split_whitespace().count() as i32)
      .max()
      .unwrap();
  }
}
