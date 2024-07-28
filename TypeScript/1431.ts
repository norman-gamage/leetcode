/**
 * 1431. Kids With the Greatest Number of Candies
 * https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
 */

function kidsWithCandies(candies: number[], extraCandies: number): boolean[] {
  const out: boolean[] = Array(candies.length);
  const max: number = Math.max(...candies);

  candies.forEach(
    (v: number, i: number) => (out[i] = v + extraCandies >= max)
  );

  return out;
}
