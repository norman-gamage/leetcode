/**
 * 2485. Find the Pivot Integer
 * https://leetcode.com/problems/find-the-pivot-integer/
 */

function pivotInteger(n: number): number {
  let sum1: number = 0;
  let sum2: number = 0;
  let out: number = -1;

  for (let i: number = 1; i <= n; i++) {
    sum1 += i;
  }

  for (let i: number = 1; i <= n; i++) {
    sum2 += i;

    if (sum1 === sum2) {
      out = i;
      break;
    }

    sum1 -= i;
  }

  return out;
}
