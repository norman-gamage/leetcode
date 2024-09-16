/**
 * 1614. Maximum Nesting Depth of the Parentheses
 * https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/
 */

function maxDepth(s: string): number {
  let cnt: number = 0;
  let out: number = 0;

  s.split("").forEach((c: string) => {
    switch (c) {
      case "(": {
        cnt += 1;

        if (cnt > out) {
          out = cnt;
        }

        break;
      }

      case ")": {
        cnt -= 1;
        break;
      }

      default: {
        break;
      }
    }
  });

  return out;
}
