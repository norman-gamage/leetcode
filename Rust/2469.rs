/**
 * 2469. Convert the Temperature
 * https://leetcode.com/problems/convert-the-temperature/
 */

impl Solution {
  pub fn convert_temperature(celsius: f64) -> Vec<f64> {
    let mut out: Vec<f64> = Vec::new();

    out.push(celsius + 273.15);
    out.push(celsius * 1.8 + 32.0);

    return out;
  }
}
