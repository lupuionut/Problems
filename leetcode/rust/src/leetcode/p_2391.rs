// 2391. Minimum Amount of Time to Collect Garbage
// -----------------------------------------------

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        // metal -> 0 , paper -> 1, glass -> 2
        let mut last_appearance: [usize; 3] = [0, 0, 0];
        let mut garbages = vec![vec![0; 3]; garbage.len()];
        let mut ans = 0;

        garbage.iter().enumerate().for_each(|(i, s)| {
            s.chars().for_each(|c| {
                if c == 'M' {
                    last_appearance[0] = i;
                    garbages[i][0] += 1;
                } else if c == 'P' {
                    last_appearance[1] = i;
                    garbages[i][1] += 1;
                } else {
                    last_appearance[2] = i;
                    garbages[i][2] += 1;
                }
            });
        });

        // route for each truck, 0 metal, 1 paper, 2 glass
        for i in 0..3 {
            for j in 0..=last_appearance[i] {
                ans += garbages[j][i];
                if (j < last_appearance[i]) {
                    ans += travel[j];
                }
            }
        }

        ans
    }
}
