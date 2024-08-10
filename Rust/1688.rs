/**
 * 1688. Count of Matches in Tournament
 * https://leetcode.com/problems/count-of-matches-in-tournament/
 */

impl Solution {
  pub fn number_of_matches(n: i32) -> i32 {
    let mut n: i32 = n;
    let mut out: i32 = 0;

    while n > 1 {
      let carry: i32 = n % 2;
      n /= 2;
      out += n + carry;
    }

    return out;
  }
}
