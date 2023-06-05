// 167. Two Sum II - Input Array Is Sorted
// ---------------------------------------

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; 2];

        for i in 0..numbers.len() {
            let search = target - numbers[i];
            let left = &numbers[0..i];
            let right = &numbers[i + 1..];

            match left.binary_search(&search) {
                Ok(id) => {
                    ans[0] = i as i32 + 1;
                    ans[1] = id as i32 + 1;
                    break;
                }
                _ => {}
            }

            match right.binary_search(&search) {
                Ok(id) => {
                    ans[0] = i as i32 + 1;
                    ans[1] = (i + 1 + id) as i32 + 1;
                    break;
                }
                _ => {}
            }
        }
        ans
    }
}
