/**
 * 3146. Permutation Difference between Two Strings
 * https://leetcode.com/problems/permutation-difference-between-two-strings/
 */

function findPermutationDifference(s: string, t: string): number {
  let out: number = 0;

  s.split("").forEach((v: string, i: number) => {
    out += Math.abs(i - t.indexOf(v));
  });

  return out;
}
