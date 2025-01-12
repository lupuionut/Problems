// 2116. Check if a Parentheses String Can Be Valid
// ------------------------------------------------

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let locked = locked.chars().collect::<Vec<_>>();
        let mut stack_locked = vec![];
        let mut stack_unlocked = vec![];

        if s.len() % 2 != 0 {
            return false;
        }

        for i in 0..s.len() {
            if locked[i] == '0' {
                stack_unlocked.push(i);
            } else {
                if s[i] == '(' {
                    stack_locked.push(i);
                } else {
                    if stack_locked.len() > 0 {
                        stack_locked.pop();
                    } else if stack_unlocked.len() > 0 {
                        stack_unlocked.pop();
                    } else {
                        return false;
                    }
                }
            }
        }

        while stack_locked.len() > 0 {
            if stack_unlocked.len() == 0 {
                break;
            }
            let last_locked = stack_locked.last().unwrap();
            let last_unlocked = stack_unlocked.last().unwrap();
            if last_unlocked > last_locked {
                stack_locked.pop();
                stack_unlocked.pop();
            } else {
                break;
            }
        }

        stack_locked.len() == 0
    }
}
