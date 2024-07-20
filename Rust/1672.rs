/**
 * 1672. Richest Customer Wealth
 * https://leetcode.com/problems/richest-customer-wealth/
 */

impl Solution {
  pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    pub fn sum(account: Vec<i32>) -> i32 {
      return account.iter().fold(0, |a, b| a + b);
    }

    let account_sum: Vec<i32> = accounts
      .iter()
      .map(|x| sum(x.to_vec()))
      .collect::<Vec<_>>();

    return *account_sum.iter().max().unwrap();
  }
}
