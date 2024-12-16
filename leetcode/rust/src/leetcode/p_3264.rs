// 3264. Final Array State After K Multiplication Operations I
// -----------------------------------------------------------

use std::collections::BinaryHeap;
impl Solution {
    pub fn get_final_state(nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        let mut heap = BinaryHeap::new();
        nums.into_iter().enumerate().for_each(|(k, v)| {
            let k = -(k as i32);
            let v = -v;
            heap.push((v, k));
        });

        while k > 0 {
            if let Some((v, k)) = heap.pop() {
                let v = -v * multiplier;
                heap.push((-v, k));
            }
            k -= 1;
        }

        for (v, k) in heap.iter() {
            let k = -k;
            ans[k as usize] = -v;
        }
        ans
    }
}
