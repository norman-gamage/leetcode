/**
 * 1221. Split a String in Balanced Strings
 * https://leetcode.com/problems/split-a-string-in-balanced-strings/
 */

function balancedStringSplit(s: string): number {
  let out: number = 0;
  let sum: number = 0;

  for (let i: number = 0; i < s.length; i++) {
    sum += s[i] === "R" ? 1 : -1;
    if (sum === 0) out += 1;
  }

  return out;
}
