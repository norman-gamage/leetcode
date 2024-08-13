/**
 * 1342. Number of Steps to Reduce a Number to Zero
 * https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
 */

function numberOfSteps(num: number): number {
  let out: number = 0;

  while (num !== 0) {
    num = num % 2 === 0 ? Math.floor(num / 2) : num - 1;
    out += 1;
  }

  return out;
}
