/**
 * 2723. Add Two Promises
 * https://leetcode.com/problems/add-two-promises/
 */

type P = Promise<number>;

async function addTwoPromises(promise1: P, promise2: P): P {
  return (await promise1) + (await promise2);
}
