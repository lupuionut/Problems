// 1524. Number of Sub-arrays With Odd Sum
// ---------------------------------------

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut pref_sum = 0;
        let mut even_count = 0;
        let mut odd_count = 0;
        let mut total = 0;

        for i in 0..arr.len() {
            pref_sum += arr[i];

            if pref_sum % 2 == 1 {
                total += 1 + even_count;
                total %= 1_000_000_007;
                odd_count += 1;
            } else {
                total += odd_count;
                total %= 1_000_000_007;
                even_count += 1;
            }
        }

        total % 1_000_000_007
    }
}
