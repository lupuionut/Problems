// 79. Word Search
// ---------------

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn traverse(
            board: &Vec<Vec<char>>,
            word: &Vec<char>,
            w: usize,
            i: i32,
            j: i32,
            visited: &mut Vec<Vec<bool>>,
        ) -> bool {
            if w == word.len() {
                return true;
            }
            if i < 0 || i >= board.len() as i32 || j < 0 || j >= board[i as usize].len() as i32 {
                return false;
            }
            if visited[i as usize][j as usize] == true {
                return false;
            }

            visited[i as usize][j as usize] = true;
            let mut ans = false;
            if board[i as usize][j as usize] == word[w] {
                ans |= traverse(board, word, w + 1, i - 1, j, visited);
                ans |= traverse(board, word, w + 1, i + 1, j, visited);
                ans |= traverse(board, word, w + 1, i, j - 1, visited);
                ans |= traverse(board, word, w + 1, i, j + 1, visited);
            }
            visited[i as usize][j as usize] = false;
            ans
        }

        let word = word.chars().collect::<Vec<char>>();
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let mut ans = false;

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if traverse(&board, &word, 0, i as i32, j as i32, &mut visited) == true {
                    return true;
                }
            }
        }
        false
    }
}
