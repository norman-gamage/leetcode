/**
 * 2666. Allow One Function Call
 * https://leetcode.com/problems/allow-one-function-call/
 */

type JSONValue =
  | null
  | boolean
  | number
  | string
  | JSONValue[]
  | { [key: string]: JSONValue };
type OnceFn = (...args: JSONValue[]) => JSONValue | undefined;

function once(fn: Function): OnceFn {
  let lock: boolean = false;

  return function (...args) {
    if (lock) return undefined;
    lock = true;
    return fn(...args);
  };
}
