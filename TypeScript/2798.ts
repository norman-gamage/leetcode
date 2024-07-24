/**
 * 2798. Number of Employees Who Met the Target
 * https://leetcode.com/problems/number-of-employees-who-met-the-target/
 */

function numberOfEmployeesWhoMetTarget(
  hours: number[],
  target: number
): number {
  return hours.filter((h) => h >= target).length;
}
