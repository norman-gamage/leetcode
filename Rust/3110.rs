/**
 * 3110. Score of a String
 * https://leetcode.com/problems/score-of-a-string/
 */

impl Solution {
  pub fn score_of_string(s: String) -> i32 {
    let mut score: i32 = 0;
    let mut prev_char_code: i32 = 0;

    for (i, c) in s.char_indices() {
      let curr_char_code = c as i32;

      if (i > 0) {
        score = score + (prev_char_code - curr_char_code).abs();
      }

      prev_char_code = curr_char_code;
    }

    return score;
  }
}
