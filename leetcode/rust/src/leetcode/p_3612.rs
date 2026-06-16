// 3612. Process String with Special Operations I
// ----------------------------------------------
use std::collections::VecDeque;
#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right
}
impl Solution {
    pub fn process_str(s: String) -> String {
        let mut q: VecDeque<char> = VecDeque::new();
        let mut d = Direction::Left;

        s.chars().for_each(|c| {
            match (c, &d) {
                ('*', Direction::Right) => {
                    q.pop_front();
                },
                ('*', Direction::Left) => {
                    q.pop_back();
                },
                ('#', Direction::Right) => {
                    let curr = q.clone();
                    for i in (0..curr.len()).rev() {
                        q.push_front(curr[i]);
                    }
                },
                ('#', Direction::Left) => {
                    for i in 0..q.len() {
                        q.push_back(q[i]);
                    }
                },
                ('%', Direction::Right) => {
                    if q.len() > 0 {
                        d = Direction::Left;
                    }
                },
                ('%', Direction::Left) => {
                    if q.len() > 0 {
                        d = Direction::Right;
                    }
                },
                (_, Direction::Right) => {
                    q.push_front(c);
                },
                (_, Direction::Left) => {
                    q.push_back(c);
                },
            }
        });

        if d == Direction::Left {
            q.iter().collect::<String>()
        } else {
            q.iter().rev().collect::<String>()
        }
    }
}
