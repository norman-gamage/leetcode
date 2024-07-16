/**
 * 2627. Debounce
 * https://leetcode.com/problems/debounce/
 */

type F = (...args: number[]) => void;

function debounce(fn: F, t: number): F {
  let timer: ReturnType<typeof setTimeout> = null;

  return function (...args) {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), t);
  };
}
