// 733. Flood Fill
// ---------------

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let mut visited = vec![vec![false; image[0].len()]; image.len()];

        fn fill(
            image: &mut Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
            x: i32,
            y: i32,
            required: i32,
            desired: i32,
        ) {
            visited[x as usize][y as usize] = true;

            if image[x as usize][y as usize] == required {
                image[x as usize][y as usize] = desired;
            }

            let d = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mx = image.len() as i32;
            let my = image[0].len() as i32;

            for (dx, dy) in d {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && ny >= 0 && nx < mx && ny < my {
                    if image[nx as usize][ny as usize] == required
                        && visited[nx as usize][ny as usize] == false
                    {
                        fill(image, visited, nx, ny, required, desired);
                    }
                }
            }
        }
        let required = image[sr as usize][sc as usize];
        fill(&mut image, &mut visited, sr, sc, required, color);
        image
    }
}
