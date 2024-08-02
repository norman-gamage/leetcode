/**
 * 2859. Sum of Values at Indices With K Set Bits
 * https://leetcode.com/problems/sum-of-values-at-indices-with-k-set-bits/
 */

function sumIndicesWithKSetBits(nums: number[], k: number): number {
  let out: number = 0;

  nums.forEach((v: number, i: number) => {
    let setBits: number = 0;

    while (i > 0 && setBits <= k) {
      setBits += i & 1;
      i >>= 1;
    }

    if (k === setBits) out += v;
  });

  return out;
}
