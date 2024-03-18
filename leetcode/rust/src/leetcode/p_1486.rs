// 1486. XOR Operation in an Array
// -------------------------------

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut k = 0;
        let mut ans = 0;
        while k < n {
            ans ^= (start + k * 2);
            k += 1;
        }

        ans
    }
}
