/**
 * 1773. Count Items Matching a Rule
 * https://leetcode.com/problems/count-items-matching-a-rule/
 */

impl Solution {
  pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let mut idx: usize = 0;
    let mut out: i32 = 0;

    if rule_key == "color" {
      idx = 1;
    } else if rule_key == "name" {
      idx = 2;
    }

    for v in items {
      if rule_value == v[idx] {
        out += 1;
      }
    }

    return out;
  }
}
