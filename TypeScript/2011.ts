/**
 * 2011. Final Value of Variable After Performing Operations
 * https://leetcode.com/problems/final-value-of-variable-after-performing-operations/
 */

function finalValueAfterOperations(operations: string[]): number {
  let out: number = 0;

  operations.forEach((o: string) => {
    out += o === "++X" || o === "X++" ? 1 : -1;
  });

  return out;
}
