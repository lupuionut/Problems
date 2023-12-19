// 661. Image Smoother
// -------------------

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; img[0].len()]; img.len()];
        let directions = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ];
        for i in 0..img.len() {
            for j in 0..img[0].len() {
                let mut total = img[i][j];
                let mut n = 1;
                for (dx, dy) in &directions {
                    let nx = (j as i32) + dx;
                    let ny = (i as i32) + dy;
                    if nx < 0 || ny < 0 || nx as usize >= img[0].len() || ny as usize >= img.len() {
                        continue;
                    }
                    total += img[ny as usize][nx as usize];
                    n += 1;
                }
                ans[i][j] = total / n;
            }
        }
        ans
    }
}
