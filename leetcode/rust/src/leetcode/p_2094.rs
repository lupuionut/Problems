// 2094. Finding 3-Digit Even Numbers
// ----------------------------------

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut keys = vec![-1; 1000];
        for i in 0..digits.len() {
            if keys[digits[i] as usize] == -1 {
                keys[digits[i] as usize] = 0;
            }
            keys[digits[i] as usize] += 1;
        }

        for i in 100..1000 {
            if i % 2 == 0 {
                // 1, 1, 1
                let a = (i / 100) as usize;
                keys[a] -= 1;
                let b = ((i / 10) % 10) as usize;
                keys[b] -= 1;
                let c = ((i % 10) % 10) as usize;
                keys[c] -= 1;

                if keys[a] >= 0 && keys[b] >= 0 && keys[c] >= 0 {
                    ans.push(i);
                    keys[a] += 1;
                    keys[b] += 1;
                    keys[c] += 1;
                    continue;
                } else {
                    keys[a] += 1;
                    keys[b] += 1;
                    keys[c] += 1;
                }

                // 2, 1
                let a = (i / 10) as usize;
                keys[a] -= 1;
                let b = ((i % 10) % 10) as usize;
                keys[b] -= 1;
                if keys[a] >= 0 && keys[b] >= 0 && a >= 10 {
                    ans.push(i);
                    keys[a] += 1;
                    keys[b] += 1;
                    continue;
                } else {
                    keys[a] += 1;
                    keys[b] += 1;
                }

                // 1, 2
                let a = (i / 100) as usize;
                keys[a] -= 1;
                let b = (i % 100) as usize;
                keys[b] -= 1;
                if keys[a] >= 0 && keys[b] >= 0 && b >= 10 {
                    ans.push(i);
                    keys[a] += 1;
                    keys[b] += 1;
                    continue;
                } else {
                    keys[a] += 1;
                    keys[b] += 1;
                }
            }
        }

        ans
    }
}
