/**
 * 2894. Divisible and Non-divisible Sums Difference
 * https://leetcode.com/problems/divisible-and-non-divisible-sums-difference/
 */

function differenceOfSums(n: number, m: number): number {
  let out: number = 0;

  for (let i: number = 1; i <= n; i++) {
    out += i % m !== 0 ? i : -i;
  }

  return out;
}
