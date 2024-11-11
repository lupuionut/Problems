// 2601. Prime Subtraction Operation
// ---------------------------------

impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        // generate primes
        let mut primes = vec![true; 1001];
        primes[0] = false;
        primes[1] = false;
        for i in 2..1000 {
            for j in i..1000 {
                if (i * j) <= 1000 {
                    primes[i * j] = false;
                }
            }
        }

        let primes = primes
            .iter()
            .enumerate()
            .map(|(k, &n)| {
                if (n == true) {
                    return k as i32;
                } else {
                    return 0;
                }
            })
            .filter(|&n| n > 0)
            .collect::<Vec<_>>();

        // binary search for the maximum prime we can use to reduce the current number
        for i in 0..nums.len() {
            let target = if i > 0 {
                nums[i] - nums[i - 1] - 1
            } else {
                nums[i] - 1
            };
            let mut left = 0;
            let mut right = (primes.len() - 1) as i32;
            let mut highest = 0;

            while left <= right {
                let middle = left + (right - left) / 2;
                if primes[middle as usize] == target {
                    highest = primes[middle as usize];
                    break;
                }
                if primes[middle as usize] > target {
                    right = middle - 1;
                } else {
                    highest = highest.max(primes[middle as usize]);
                    left = middle + 1;
                }
            }
            nums[i] = nums[i] - highest;

            if i > 0 {
                if nums[i] <= nums[i - 1] {
                    return false;
                }
            }
        }

        true
    }
}
