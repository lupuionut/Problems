// 2337. Move Pieces to Obtain a String
// ------------------------------------

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let chars1 = start.chars().collect::<Vec<_>>();
        let chars2 = target.chars().collect::<Vec<_>>();
        let mut left = 0;
        let mut right = 0;

        if chars1.len() != chars2.len() {
            return false;
        }

        let mut l = 0;
        let mut r = 0;

        for i in 0..chars1.len() {
            if chars1[i] == 'L' {
                l += 1;
            } else if chars1[i] == 'R' {
                r += 1;
            }
            if chars2[i] == 'L' {
                l -= 1;
            } else if chars2[i] == 'R' {
                r -= 1;
            }
        }

        if l != 0 || r != 0 {
            return false;
        }

        for i in 0..chars1.len() {
            match (chars2[i], chars1[i]) {
                ('L', 'R') | ('R', 'L') => {
                    return false;
                }
                ('L', 'L') => {
                    if right != 0 {
                        return false;
                    }
                }
                ('R', 'R') => {
                    if left != 0 {
                        return false;
                    }
                }
                ('L', '_') => {
                    left -= 1;
                    right = 0;
                }
                ('R', '_') => {
                    if right <= 0 {
                        return false;
                    }
                    right -= 1;
                    left = 0;
                }
                ('_', 'L') => {
                    if left < 0 {
                        left += 1;
                    }
                }
                ('_', 'R') => {
                    if left < 0 {
                        return false;
                    }
                    right += 1;
                }
                (_, _) => {}
            }
        }

        if left != 0 || right != 0 {
            return false;
        }
        true
    }
}
