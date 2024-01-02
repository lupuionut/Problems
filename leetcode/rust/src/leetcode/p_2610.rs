// 2610. Convert an Array Into a 2D Array With Conditions
// ------------------------------------------------------

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut values = vec![0; 201];
        let mut max = 0;
        nums.iter().for_each(|&n| {
            values[n as usize] += 1;
            max = max.max(values[n as usize]);
        });

        let mut ans = vec![vec![]; max as usize];

        while max >= 1 {
            for i in 0..201 {
                if values[i] != 0 {
                    ans[max as usize - 1].push(i as i32);
                    values[i] -= 1;
                }
            }
            max -= 1;
        }

        ans
    }
}
