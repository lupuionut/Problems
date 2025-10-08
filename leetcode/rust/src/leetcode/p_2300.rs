// 2300. Successful Pairs of Spells and Potions
// --------------------------------------------
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut ans = vec![0; spells.len()];
        let n = potions.len();
        potions.sort();
        for i in 0..spells.len() {
            let mut target = success / spells[i] as i64;
            if (success % spells[i] as i64) != 0 {
                target += 1;
            }
            let mut l = 0;
            let mut r = n - 1;

            while l < r {
                let mid = (l + r) / 2;
                if (potions[mid] as i64) < target {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            if l == n - 1 {
                if potions[l] as i64 >= target {
                    ans[i] = 1 as i32;
                }
            } else {
                ans[i] = (n - l) as i32;
            }
        }
        ans
    }
}
