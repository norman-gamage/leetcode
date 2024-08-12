/**
 * 1816. Truncate Sentence
 * https://leetcode.com/problems/truncate-sentence/
 */

impl Solution {
  pub fn truncate_sentence(s: String, k: i32) -> String {
    let mut out: Vec<&str> = s.split(" ").collect();
    out.truncate(k as usize);
    return out.join(" ");
  }
}
