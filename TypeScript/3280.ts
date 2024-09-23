/**
 * 3280. Convert Date to Binary
 * https://leetcode.com/problems/convert-date-to-binary/
 */

function convertDateToBinary(date: string): string {
  return date
    .split("-")
    .map((d: string) => Number(d).toString(2))
    .join("-");
}
