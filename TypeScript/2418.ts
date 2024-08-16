/**
 * 2418. Sort the People
 * https://leetcode.com/problems/sort-the-people/
 */

type HeightName = [number, string];

function sortPeople(names: string[], heights: number[]): string[] {
  const zipped: HeightName[] = heights.map((v: number, i: number) => [
    v,
    names[i],
  ]);
  zipped.sort((a: HeightName, b: HeightName) => Number(b[0]) - Number(a[0]));
  return zipped.map((v: HeightName) => String(v[1]));
}
