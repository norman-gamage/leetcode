/**
 * 1021. Remove Outermost Parentheses
 * https://leetcode.com/problems/remove-outermost-parentheses/
 */

function removeOuterParentheses(s: string): string {
  let cnt: number = 0;
  let out: string = "";

  for (let i: number = 0; i < s.length; i++) {
    let c: string = s[i];

    cnt += c === "(" ? 1 : -1;

    if ((c === "(" && cnt === 1) || (c === ")" && cnt === 0)) {
      continue;
    }

    out += c;
  }

  return out;
}
