/**
 * 2413. Smallest Even Multiple
 * https://leetcode.com/problems/smallest-even-multiple/
 */

function smallestEvenMultiple(n: number): number {
  return n % 2 === 0 ? n : n * 2;
}
