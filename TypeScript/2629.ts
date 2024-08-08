/**
 * 2629. Function Composition
 * https://leetcode.com/problems/function-composition/
 */

type F = (x: number) => number;

function compose(functions: F[]): F {
  return function (x: number) {
    functions.reverse().map((f: F) => (x = f(x)));
    return x;
  };
}
