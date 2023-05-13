/*
  2623. Memoize
  -------------
*/

function memoize(fn) {
    const c = new Map();
    return function(...args) {
        let key = 1;
        for (const a of args) {
          key += '-' + a;
        }
        if (c.has(key)) {
          return c.get(key);
        }
        const result = fn(...args);
        c.set(key, result);
        return result
    }
}
