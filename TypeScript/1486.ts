/**
 * 1486. XOR Operation in an Array
 * https://leetcode.com/problems/xor-operation-in-an-array/
 */

function xorOperation(n: number, start: number): number {
  let out: number = 0;

  for (let i: number = 0; i < n; i++) {
    out ^= start;
    start += 2;
  }

  return out;
}
