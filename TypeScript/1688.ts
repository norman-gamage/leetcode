/**
 * 1688. Count of Matches in Tournament
 * https://leetcode.com/problems/count-of-matches-in-tournament/
 */

function numberOfMatches(n: number): number {
  let out: number = 0;

  while (n > 1) {
    const carry: number = n % 2;
    n = Math.floor(n / 2);
    out += n + carry;
  }

  return out;
}
