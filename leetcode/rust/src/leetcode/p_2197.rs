// 2197. Replace Non-Coprime Numbers in Array
// ------------------------------------------

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                return a;
            }
            if a == 0 {
                return b;
            }
            if a > b {
                return gcd(a % b, b);
            } else {
                return gcd(b % a, a);
            }
        }
        fn lcm(a: i32, b: i32) -> i32 {
            let r = (a as i64 * b as i64) / gcd(a, b) as i64;
            r as i32
        }
        let mut ans = vec![];
        for &num in &nums {
            ans.push(num);
            while ans.len() >= 2 {
                let n = ans.len();
                if gcd(ans[n - 1], ans[n - 2]) > 1 {
                    let a = ans.pop().unwrap();
                    let b = ans.pop().unwrap();
                    ans.push(lcm(a, b));
                } else {
                    break;
                }
            }
        }
        ans
    }
}
