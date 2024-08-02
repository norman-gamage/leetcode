/**
 * 1281. Subtract the Product and Sum of Digits of an Integer
 * https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
 */

impl Solution {
  pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut n: i32 = n;
    let mut product: i32 = 1;
    let mut sum: i32 = 0;

    while n > 0 {
      let d: i32 = n % 10;

      product *= d;
      sum += d;

      n /= 10;
    }

    return product - sum;
  }
}
