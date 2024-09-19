/**
 * 1021. Remove Outermost Parentheses
 * https://leetcode.com/problems/remove-outermost-parentheses/
 */

impl Solution {
  pub fn remove_outer_parentheses(s: String) -> String {
    let mut cnt: i32 = 0;
    let mut out: String = "".to_string();

    for c in s.chars() {
      if c == '(' {
        cnt += 1;
      } else {
        cnt -= 1;
      }

      if (c == '(' && cnt == 1) || (c == ')' && cnt == 0) {
        continue;
      }

      out.push(c);
    }

    return out;
  }
}
