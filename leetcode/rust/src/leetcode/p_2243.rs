// 2243. Calculate Digit Sum of a String
// -------------------------------------
impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut s = s.chars().map(|c| c as usize - 48).collect::<Vec<usize>>();

        fn to_digits(mut n: usize) -> Vec<usize> {
            let mut ans = vec![];
            if n == 0 {
                ans.push(0);
                return ans;
            }

            while n > 0 {
                ans.push(n % 10);
                n /= 10;
            }
            ans.into_iter().rev().collect::<Vec<usize>>()
        }

        loop {
            if s.len() <= k as usize {
                break;
            } 
            let mut t = vec![];
            s.chunks(k as usize).for_each(|c| {
                let mut total = 0;
                for &v in c {
                    total += v;
                }
                for d in to_digits(total) {
                    t.push(d);
                }
            });
            s = t;
        }


        s.iter().map(|v| v.to_string()).collect::<String>()
    }
}
