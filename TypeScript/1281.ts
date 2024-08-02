/**
 * 1281. Subtract the Product and Sum of Digits of an Integer
 * https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
 */

function subtractProductAndSum(n: number): number {
  let product: number = 1;
  let sum: number = 0;

  while (n > 0) {
    let d: number = n % 10;

    product *= d;
    sum += d;

    n = Math.floor(n / 10);
  }

  return product - sum;
}
