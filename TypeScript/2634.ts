/**
 * 2634. Filter Elements from Array
 * https://leetcode.com/problems/filter-elements-from-array/
 */

type Fn = (n: number, i: number) => any;

function filter(arr: number[], fn: Fn): number[] {
  const out: number[] = [];

  arr.forEach((v: number, i: number) => {
    if (fn(v, i)) out.push(v);
  });

  return out;
}
