// 3783. Mirror Distance of an Integer
// -----------------------------------
impl Solution {
    pub fn mirror_distance(mut n: i32) -> i32 {
        let mut digits = vec![];
        let a = n;
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        let mut b = 0;
        for i in 0..digits.len() {
            b *= 10;
            b += digits[i];
        }
        (a - b).abs()
    }
}
