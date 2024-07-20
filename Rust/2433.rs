/**
 * 2433. Find The Original Array of Prefix Xor
 * https://leetcode.com/problems/find-the-original-array-of-prefix-xor/submissions/1327260397/
 */

impl Solution {
  pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    out.push(pref[0]);

    for i in 1..pref.len() {
      out.push(pref[i] ^ pref[i - 1]);
    }

    return out;
  }
}
