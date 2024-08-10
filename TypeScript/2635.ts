/**
 * 2635. Apply Transform Over Each Element in Array
 * https://leetcode.com/problems/apply-transform-over-each-element-in-array/
 */

function map(arr: number[], fn: (n: number, i: number) => number): number[] {
  return arr.map((v: number, i: number) => fn(v, i));
}
