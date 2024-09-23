/**
 * 3280. Convert Date to Binary
 * https://leetcode.com/problems/convert-date-to-binary/
 */

impl Solution {
  pub fn convert_date_to_binary(date: String) -> String {
    let mut arr: Vec<&str> = date.split("-").collect();
    let mut out: Vec<String> = Vec::new();

    for a in arr {
      out.push(format!("{:b}", a.parse::<i32>().unwrap()));
    }

    return out.join("-");
  }
}
