// 1496. Path Crossing
// -------------------

use std::collections::HashSet;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut current = (0, 0);
        visited.insert(current);

        for c in path.chars() {
            current = match c {
                'N' => (current.0, current.1 + 1),
                'E' => (current.0 + 1, current.1),
                'S' => (current.0, current.1 - 1),
                'W' => (current.0 - 1, current.1),
                _ => panic!("unreacheable"),
            };
            if visited.contains(&current) {
                return true;
            }
            visited.insert(current);
        }

        false
    }
}
