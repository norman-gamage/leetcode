/**
 * 1684. Count the Number of Consistent Strings
 * https://leetcode.com/problems/count-the-number-of-consistent-strings/
 */

impl Solution {
  pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut out: i32 = 0;

    for word in words {
      let mut flag: bool = false;

      for w in word.chars() {
        match allowed.find(w) {
          Some(_) => {
            continue;
          }
          None => {
            flag = true;
            break;
          }
        }
      }

      if !flag {
        out += 1;
      }
    }

    return out;
  }
}
