/**
 * 1773. Count Items Matching a Rule
 * https://leetcode.com/problems/count-items-matching-a-rule/
 */

function countMatches(
  items: string[][],
  ruleKey: string,
  ruleValue: string
): number {
  const idx: number = ruleKey === "color" ? 1 : ruleKey === "name" ? 2 : 0;
  let out: number = 0;

  items.forEach((v: string[]) => {
    if (ruleValue === v[idx]) {
      out += 1;
    }
  });

  return out;
}
