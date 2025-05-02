// 838. Push Dominoes
// ------------------

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut ans = dominoes.clone().chars().collect::<Vec<char>>();
        let mut intval = (-1, -1);
        dominoes
            .chars()
            .enumerate()
            .for_each(|(idx, c)| match (intval, c) {
                ((-1, -1), 'L') => {
                    let mut j = idx as i32 - 1;
                    while j >= 0 && ans[j as usize] == '.' {
                        ans[j as usize] = 'L';
                        j -= 1;
                    }
                }
                ((-1, -1), 'R') => {
                    intval.0 = idx as i32;
                }
                ((l, -1), 'L') => {
                    let mut i = l + 1;
                    let mut j = (idx as i32) - 1;
                    while i < j {
                        ans[i as usize] = 'R';
                        ans[j as usize] = 'L';
                        i += 1;
                        j -= 1;
                    }
                    intval = (-1, -1);
                }
                ((l, -1), 'R') => {
                    let mut i = l + 1;
                    let mut j = (idx as i32);
                    while i < j {
                        ans[i as usize] = 'R';
                        i += 1;
                    }
                    intval.0 = idx as i32;
                }
                ((-1, _), 'L') | ((_, _), '.') => {}
                _ => {
                    panic!("impossible")
                }
            });
        if intval.0 != -1 {
            for i in intval.0 + 1..dominoes.len() as i32 {
                ans[i as usize] = 'R';
            }
        }
        ans.iter().collect()
    }
}
