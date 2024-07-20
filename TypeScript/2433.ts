/**
 * 2433. Find The Original Array of Prefix Xor
 * https://leetcode.com/problems/find-the-original-array-of-prefix-xor/submissions/1327260397/
 */

function findArray(pref: number[]): number[] {
  let out = [pref[0]];

  for (let i: number = 1; i < pref.length; i++) {
    out.push(pref[i] ^ pref[i - 1]);
  }

  return out;
}
