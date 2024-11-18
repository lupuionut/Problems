// 1652. Defuse the Bomb
// ---------------------

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut ans = vec![0; n];

        for i in 0..n {
            let mut t = 0;
            let mut kk = k;
            if k > 0 {
                if i + k as usize >= n {
                    for j in i + 1..n {
                        t += code[j];
                        kk -= 1;
                    }
                    for j in 0..kk as usize {
                        t += code[j];
                    }
                } else {
                    for j in i + 1..=i + k as usize {
                        t += code[j];
                    }
                }
            } else if k < 0 {
                if i as i32 + k < 0 {
                    for j in 0..i {
                        kk += 1;
                        t += code[j];
                    }
                    for j in n as i32 + kk..n as i32 {
                        t += code[j as usize];
                    }
                } else {
                    for j in i + k as usize..i {
                        t += code[j];
                    }
                }
            }

            ans[i] = t;
        }

        ans
    }
}
