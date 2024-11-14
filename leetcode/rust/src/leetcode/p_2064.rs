// 2064. Minimized Maximum of Products Distributed to Any Store
// ------------------------------------------------------------

impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = *quantities.iter().max().unwrap();
        while left < right {
            let mut middle = (left + right) / 2;
            if is_good(middle, n, &quantities) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        fn is_good(k: i32, n: i32, quantities: &Vec<i32>) -> bool {
            let mut m = 0;

            for i in 0..quantities.len() {
                let d = quantities[i] / k;
                let r = quantities[i] % k;
                if r > 0 {
                    m += (d + 1);
                } else {
                    m += d;
                }
            }

            if m <= n {
                true
            } else {
                false
            }
        }

        left
    }
}
