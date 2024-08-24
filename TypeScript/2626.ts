/**
 * 2626. Array Reduce Transformation
 * https://leetcode.com/problems/array-reduce-transformation/
 */

type Fn = (accum: number, curr: number) => number;

function reduce(nums: number[], fn: Fn, init: number): number {
  let accum: number = init;

  nums.forEach((curr: number) => {
    accum = fn(accum, curr);
  });

  return accum;
}
