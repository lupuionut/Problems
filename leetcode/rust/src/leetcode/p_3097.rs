// 3097. Shortest Subarray With OR at Least K II
// ---------------------------------------------

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut best = i32::MAX;
        let mut bits = vec![0; 32];

        for right in 0..nums.len() {
            Self::add_bits(nums[right], &mut bits);
            while Self::sum_bits(&bits) >= k && left <= right {
                best = best.min((right - left + 1) as i32);
                Self::sub_bits(nums[left], &mut bits);
                left += 1;
            }
        }

        if best == i32::MAX {
            return -1;
        }
        best
    }

    pub fn add_bits(mut n: i32, bits: &mut Vec<i32>) {
        for i in 0..32 {
            if n == 0 {
                break;
            }
            bits[i] += n & 1;
            n = n >> 1;
        }
    }

    pub fn sub_bits(mut n: i32, bits: &mut Vec<i32>) {
        for i in 0..32 {
            if n == 0 {
                break;
            }
            bits[i] -= n & 1;
            n = n >> 1;
        }
    }

    pub fn sum_bits(bits: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..32 {
            if bits[i] > 0 {
                sum |= (1 << i);
            }
        }
        sum
    }
}
