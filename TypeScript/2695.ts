/**
 * 2695. Array Wrapper
 * https://leetcode.com/problems/array-wrapper/
 */

class ArrayWrapper {
  private obj: number[];

  constructor(nums: number[]) {
    this.obj = nums;
  }

  valueOf(): number {
    return this.obj.reduce((a, b) => a + b, 0);
  }

  toString(): string {
    return `[${this.obj}]`;
  }
}
