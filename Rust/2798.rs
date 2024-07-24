/**
 * 2798. Number of Employees Who Met the Target
 * https://leetcode.com/problems/number-of-employees-who-met-the-target/
 */

impl Solution {
  pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    return hours
      .into_iter()
      .filter(|&h| h >= target)
      .collect::<Vec<i32>>()
      .len() as i32;
  }
}
