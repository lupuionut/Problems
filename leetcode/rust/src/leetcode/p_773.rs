// 773. Sliding Puzzle
// -------------------

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut dirs = vec![
            vec![1, 3],
            vec![0, 2, 4],
            vec![1, 5],
            vec![0, 4],
            vec![1, 3, 5],
            vec![2, 4],
        ];
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        let b = board
            .iter()
            .flatten()
            .map(|c| c.to_string())
            .collect::<String>();
        visited.insert(b.clone());
        let mut idx = 0;
        board.iter().flatten().enumerate().for_each(|(k, &v)| {
            if v == 0 {
                idx = k;
            }
        });

        q.push_back((idx, b, 0));
        while q.len() > 0 {
            if let Some((pos, hash, depth)) = q.pop_front() {
                if hash == "123450".to_string() {
                    return depth;
                }
                let directions = &dirs[pos];
                for &direction in directions {
                    let mut new_hash_arr = hash.chars().collect::<Vec<char>>();
                    let temp = new_hash_arr[pos];
                    new_hash_arr[pos] = new_hash_arr[direction];
                    new_hash_arr[direction] = temp;
                    let new_hash = new_hash_arr.iter().collect::<String>();
                    if !visited.contains(&new_hash) {
                        q.push_back((direction, new_hash.clone(), depth + 1));
                        visited.insert(new_hash);
                    }
                }
            }
        }
        -1
    }
}
