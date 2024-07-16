/**
 * 2469. Convert the Temperature
 * https://leetcode.com/problems/convert-the-temperature/
 */

function convertTemperature(celsius: number): number[] {
  return [celsius + 273.15, 32 + celsius * 1.8];
}
