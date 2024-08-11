/**
 * 1662. Check If Two String Arrays are Equivalent
 * https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
 */

impl Solution {
  pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    return word1.join("") == word2.join("");
  }
}
