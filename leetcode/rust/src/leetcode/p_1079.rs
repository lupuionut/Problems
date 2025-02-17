// 1079. Letter Tile Possibilities
// -------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut found: HashSet<Vec<char>> = HashSet::new();
        let chars = tiles.chars().collect();
        let mut curr = vec![vec![]];
        let mut used = 0;
        let full = (1 << tiles.len()) - 1;

        fn backtrack(
            chars: &Vec<char>,
            full: i32,
            used: &mut i32,
            curr: &mut Vec<Vec<char>>,
            found: &mut HashSet<Vec<char>>,
        ) {
            if *used == full {
                return;
            }
            for i in 0..chars.len() {
                // we used that position from chars
                if *used & (1 << i) != 0 {
                    continue;
                }
                // mark it as used
                *used |= (1 << i);

                for j in 0..curr.len() {
                    curr[j].push(chars[i]);
                    found.insert(curr[j].clone());
                }
                backtrack(chars, full, used, curr, found);

                // reverse
                *used ^= (1 << i);
                for j in 0..curr.len() {
                    curr[j].pop();
                }
            }
        }

        backtrack(&chars, full, &mut used, &mut curr, &mut found);
        found.len() as i32
    }
}
