/**
 * 1672. Richest Customer Wealth
 * https://leetcode.com/problems/richest-customer-wealth/
 */

function maximumWealth(accounts: number[][]): number {
  return Math.max(
    ...accounts.map((arr: number[]) =>
      arr.reduce((a: number, b: number) => a + b, 0)
    )
  );
}
