/**
 * 1108. Defanging an IP Address
 * https://leetcode.com/problems/defanging-an-ip-address/
 */

use regex::Regex;

impl Solution {
  pub fn defang_i_paddr(address: String) -> String {
    let re = Regex::new(r"\.").unwrap();
    return re.replace_all(&address, "[.]").to_string();
  }
}
