// 2491. Divide Players Into Teams of Equal Skill
// ----------------------------------------------

impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        skill.sort();
        let mut l = 0;
        let mut r = skill.len() - 1;
        let mut expected = skill[l] + skill[r];
        let mut ans = 0;
        while l < r {
            let curr = skill[l] + skill[r];
            if curr != expected {
                return -1;
            }
            ans += (skill[l] as i64) * (skill[r] as i64);
            l += 1;
            r -= 1;
        }

        ans
    }
}
