// 3867. Sum of GCD of Formed Pairs
// --------------------------------
impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let mut p_gcd = vec![0; nums.len()];
        let mut mx = 0;
        fn gcd(a: i32, b: i32) -> i64 {
            if b == 0 {
                return a as i64;
            }
            if a == 0 {
                return b as i64;
            }
            if a > b {
                return gcd(a % b, b);
            } else {
                return gcd(b % a, a);
            }
        }

        for i in 0..nums.len() {
            mx = mx.max(nums[i]);
            p_gcd[i] = gcd(nums[i], mx);
        }

        p_gcd.sort();
        let mut ans = 0;
        let n = p_gcd.len();
        for i in 0..n/2 {
            ans += gcd(p_gcd[i] as i32, p_gcd[n-i-1] as i32);
        }
        ans
    }
}
