// 2784. Check if Array is Good
// ----------------------------
impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let max: i32 = *nums.iter().max().unwrap();
        let mut vals = vec![0; (max + 1) as usize];
        nums.iter().for_each(|&n| vals[n as usize] += 1);
        let mut ans = true;
        for i in 1..vals.len() - 1 {
            ans = ans && (vals[i] == 1);
        }
        ans = ans && (vals[max as usize] == 2);
        ans
    }
}
