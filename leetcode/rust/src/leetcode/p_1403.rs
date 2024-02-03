// 1043. Partition Array for Maximum Sum
// -------------------------------------

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        fn max_sum(i: usize, arr: &Vec<i32>, k: i32, cache: &mut Vec<i32>) -> i32 {
            if i >= arr.len() {
                return 0;
            }
            if cache[i] != -1 {
                return cache[i];
            }

            let mut best = 0;
            let mut sum = 0;
            for j in 0..k as usize {
                if i + j >= arr.len() {
                    break;
                }
                best = best.max(arr[i + j]);
                sum = sum.max(max_sum(i + j + 1, arr, k, cache) + (best * (j + 1) as i32));
            }
            cache[i] = sum;
            cache[i]
        }
        let mut cache = vec![-1; arr.len()];
        max_sum(0, &arr, k, &mut cache)
    }
}
