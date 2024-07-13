/**
 * 3110. Score of a String
 * https://leetcode.com/problems/score-of-a-string/
 */

function scoreOfString(s: string): number {
  let score: number = 0;

  for (let i: number = 1; i < s.length; i++) {
    score += Math.abs(s[i - 1].charCodeAt(0) - s[i].charCodeAt(0));
  }

  return score;
}
