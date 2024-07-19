/**
 * 2715. Timeout Cancellation
 * https://leetcode.com/problems/timeout-cancellation/
 */

type JSONValue =
  | null
  | boolean
  | number
  | string
  | JSONValue[]
  | { [key: string]: JSONValue };
type Fn = (...args: JSONValue[]) => void;

function cancellable(fn: Fn, args: JSONValue[], t: number): Function {
  // 1. Delay calling `fn` (with `args`) in `t` milli-seconds
  // 2. Capture the timer identification number in `timeoutID`
  const timeoutID: ReturnType<typeof setTimeout> = setTimeout(
    () => fn(...args),
    t
  );

  // Return function that aborts delayed execution of `timeoutID`
  return () => clearTimeout(timeoutID);
}
