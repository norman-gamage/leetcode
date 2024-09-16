/**
 * 1614. Maximum Nesting Depth of the Parentheses
 * https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/
 */

impl Solution {
  pub fn max_depth(s: String) -> i32 {
    let mut cnt: i32 = 0;
    let mut out: i32 = 0;

    for c in s.chars() {
      match c {
        '(' => {
          cnt += 1;

          if cnt > out {
            out = cnt;
          }
        }

        ')' => {
          cnt -= 1;
        }

        _ => {}
      }
    }

    return out;
  }
}
