// 2022. Convert 1D Array Into 2D Array

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        if m * n != original.len() as i32 {
            return ans;
        }

        let mut iter = original.chunks(n as usize);
        while let Some(a) = iter.next() {
            ans.push(a.to_vec());
        }

        ans
    }
}
