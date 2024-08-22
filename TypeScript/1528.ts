/**
 * 1528. Shuffle String
 * https://leetcode.com/problems/shuffle-string/
 */

function restoreString(s: string, indices: number[]): string {
  const out: string[] = Array(indices.length).fill(null);

  indices.forEach((v: number, i: number) => {
    out[v] = s[i];
  });

  return out.join("");
}
