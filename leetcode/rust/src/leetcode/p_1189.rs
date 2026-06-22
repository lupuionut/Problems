// 1189. Maximum Number of Balloons
// --------------------------------
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut f = [0; 5];
        text.chars().for_each(|c| {
            match c {
                'a' => {
                    f[0] += 1;
                },
                'b' => {
                    f[1] += 1;
                },
                'l' => {
                    f[2] += 1;
                },
                'o' => {
                    f[3] += 1;
                },
                'n' => {
                    f[4] += 1;
                },
                _ => {}
            };
        });
        f[2] /= 2;
        f[3] /= 2;
        *f.iter().min().unwrap()
    }
}
