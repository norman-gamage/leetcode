/**
 * 2621. Sleep
 * https://leetcode.com/problems/sleep/
 */

async function sleep(millis: number): Promise<void> {
  return new Promise((res) => setTimeout(() => res(), millis));
}
