// 2305. Fair Distribution of Cookies
// ----------------------------------

impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let mut distributed_cookies = vec![0; k as usize];

        fn distribute(
            i: usize,
            k: i32,
            mut waiting: i32,
            cookies: &Vec<i32>,
            distributed_cookies: &mut Vec<i32>,
        ) -> i32 {
            if waiting > (cookies.len() - i) as i32 {
                return std::i32::MAX;
            }
            if i == cookies.len() {
                return *distributed_cookies.iter().max().unwrap();
            }

            let mut best = std::i32::MAX;
            for j in 0..k as usize {
                if distributed_cookies[j] == 0 {
                    waiting -= 1;
                }
                distributed_cookies[j] += cookies[i];
                best = best.min(distribute(i + 1, k, waiting, cookies, distributed_cookies));
                distributed_cookies[j] -= cookies[i];
                if distributed_cookies[j] == 0 {
                    waiting += 1;
                }
            }

            best
        }

        distribute(0, k, k, &cookies, &mut distributed_cookies)
    }
}
