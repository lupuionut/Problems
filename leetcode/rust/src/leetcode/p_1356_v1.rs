// 1356. Sort Integers by The Number of 1 Bits
// -------------------------------------------
impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort();
        let mut buckets = vec![vec![]; 20];
        for i in 0..arr.len() {
            let mut n = arr[i];
            let mut ones = 0;
            while n > 0 {
                if n % 2 == 1 {
                    ones += 1;
                }
                n /= 2;
            }
            buckets[ones as usize].push(arr[i]);
        }
        let mut ans = vec![];
        for i in 0..buckets.len() {
            for j in 0..buckets[i].len() {
                ans.push(buckets[i][j]);
            }
        }
        ans
    }
}
