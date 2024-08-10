/**
 * 2652. Sum Multiples
 * https://leetcode.com/problems/sum-multiples/
 */

function sumOfMultiples(n: number): number {
  let out: number = 0;

  for (let i: number = 0; i <= n; i++) {
    if (i % 3 === 0 || i % 5 === 0 || i % 7 === 0) {
      out += i;
    }
  }

  return out;
}
