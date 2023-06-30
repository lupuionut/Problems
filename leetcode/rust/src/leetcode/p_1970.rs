// 1970. Last Day Where You Can Still Cross
// ----------------------------------------

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut parents: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let directions = &[
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        let left = (-1, 0);
        let right = (-2, 0);
        parents.insert(left, left);
        parents.insert(right, right);

        fn find(key: (i32, i32), parents: &mut HashMap<(i32, i32), (i32, i32)>) -> (i32, i32) {
            let parent = parents.get(&key).unwrap();
            if *parent != key {
                let pparent = find(*parent, parents);
                parents.insert(key, pparent);
            }
            *parents.get(&key).unwrap()
        }

        fn union(a: (i32, i32), b: (i32, i32), parents: &mut HashMap<(i32, i32), (i32, i32)>) {
            let parent_a = find(a, parents);
            let parent_b = find(b, parents);
            if parent_a != parent_b {
                parents.insert(parent_a, parent_b);
            }
        }

        for i in 0..row {
            for j in 0..col {
                parents.insert((i, j), (i, j));
            }
        }

        for (t, coord) in cells.iter().enumerate() {
            let mut x = coord[0];
            let mut y = coord[1];
            x -= 1;
            y -= 1;

            for (dx, dy) in directions {
                let nx = x + dx;
                let ny = y + dy;

                if nx >= 0 && nx < row {
                    if ny >= 0 && ny < col {
                        if seen.contains(&(nx, ny)) {
                            union((x, y), (nx, ny), &mut parents);
                        }
                    } else {
                        if ny == -1 {
                            union((x, y), left, &mut parents);
                        }
                        if ny == col {
                            union((x, y), right, &mut parents);
                        }
                    }
                }

                if find(left, &mut parents) == find(right, &mut parents) {
                    return t as i32;
                }
            }
            seen.insert((x, y));
        }

        0
    }
}
