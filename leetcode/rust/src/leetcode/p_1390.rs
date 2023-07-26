// 1390. Four Divisors
// -------------------

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        fn sum_divisors(n: i32) -> i32 {
            let mut count = 0;
            let mut sum = 0;
            let mut i = 1;

            while i * i <= n {
                if n % i == 0 {
                    let j = n / i;
                    if i != j {
                        sum += i;
                        sum += j;
                        count += 2;
                    } else {
                        sum += i;
                        count += 1;
                    }
                }

                if count > 4 {
                    sum = 0;
                    break;
                }
                i += 1;
            }
            if count == 4 {
                sum
            } else {
                0
            }
        }

        nums.iter().for_each(|&x| ans += sum_divisors(x));
        ans
    }
}
