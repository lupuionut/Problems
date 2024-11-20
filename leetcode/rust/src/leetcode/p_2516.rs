// 2516. Take K of Each Character From Left and Right
// --------------------------------------------------

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let mut abc_r = vec![0, 0, 0];
        let mut abc_l = vec![0, 0, 0];
        let chars = s.chars().collect::<Vec<_>>();
        let mut right = chars.len() as i32;
        let mut left = 0;
        let mut sum = 0;

        if k == 0 {
            return 0;
        }
        while right > 0 && sum != 3 * k {
            right -= 1;
            let key = (chars[right as usize] as i32 - 97) as usize;
            abc_r[key] += 1;
            if abc_r[key] <= k {
                sum += 1;
            }
        }
        if sum != 3 * k {
            return -1;
        }
        let mut ans = chars.len() as i32 - right;
        for i in 0..chars.len() {
            let left_key = (chars[i] as i32 - 97) as usize;
            abc_l[left_key] += 1;
            while right < chars.len() as i32 {
                let right_key = (chars[right as usize] as i32 - 97) as usize;
                if abc_r[right_key] + abc_l[right_key] > k {
                    abc_r[right_key] -= 1;
                    right += 1;
                } else {
                    break;
                }
            }
            ans = ans.min(i as i32 + 1 + chars.len() as i32 - right);
        }

        ans
    }
}
