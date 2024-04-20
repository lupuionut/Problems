// 1992. Find All Groups of Farmland
// ---------------------------------

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut visited = vec![vec![false; land[0].len()]; land.len()];

        fn visit(
            land: &Vec<Vec<i32>>,
            i: usize,
            j: usize,
            visited: &mut Vec<Vec<bool>>,
            mx: &mut (usize, usize),
        ) {
            if i >= land.len() || j >= land[0].len() {
                return;
            }
            if land[i][j] == 0 {
                return;
            }
            visited[i][j] = true;
            *mx = (mx.0.max(i), mx.1.max(j));
            if i + 1 < land.len() && visited[i + 1][j] == false {
                visit(land, i + 1, j, visited, mx);
            }
            if j + 1 < land[0].len() && visited[i][j + 1] == false {
                visit(land, i, j + 1, visited, mx);
            }
        }

        for i in 0..land.len() {
            for j in 0..land[i].len() {
                if land[i][j] == 1 && visited[i][j] == false {
                    let mut mx = (i, j);
                    visit(&land, i, j, &mut visited, &mut mx);
                    ans.push(vec![i as i32, j as i32, mx.0 as i32, mx.1 as i32]);
                }
            }
        }

        ans
    }
}
