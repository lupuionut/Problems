// 2092. Find All People With Secret
// ---------------------------------

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut knowers = HashSet::new();
        knowers.insert(first_person);
        knowers.insert(0);
        meetings.sort_by(|a, b| a[2].cmp(&b[2]));

        let mut i = 0;
        while i < meetings.len() {
            let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
            let person1 = meetings[i][0];
            let person2 = meetings[i][1];
            let time = meetings[i][2];
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();

            if knowers.contains(&person1) && !visited.contains(&person1) {
                queue.push_back(person1);
                visited.insert(person1);
            }
            if knowers.contains(&person2) && !visited.contains(&person2) {
                queue.push_back(person2);
                visited.insert(person2);
            }
            graph
                .entry(person1)
                .and_modify(|c| c.push(person2))
                .or_insert(vec![person2]);
            graph
                .entry(person2)
                .and_modify(|c| c.push(person1))
                .or_insert(vec![person1]);

            let mut j = i + 1;

            while j < meetings.len() {
                let person1 = meetings[j][0];
                let person2 = meetings[j][1];
                let ntime = meetings[j][2];

                if ntime != time {
                    break;
                }

                graph
                    .entry(person1)
                    .and_modify(|c| c.push(person2))
                    .or_insert(vec![person2]);
                graph
                    .entry(person2)
                    .and_modify(|c| c.push(person1))
                    .or_insert(vec![person1]);

                if knowers.contains(&person1) && !visited.contains(&person1) {
                    queue.push_back(person1);
                    visited.insert(person1);
                }
                if knowers.contains(&person2) && !visited.contains(&person2) {
                    queue.push_back(person2);
                    visited.insert(person2);
                }

                j += 1;
            }

            while queue.len() > 0 {
                let curr = queue.pop_front().unwrap();
                if let Some(nodes) = graph.get(&curr) {
                    for &node in nodes {
                        if !visited.contains(&node) {
                            knowers.insert(node);
                            visited.insert(node);
                            queue.push_back(node);
                        }
                    }
                }
            }

            i = j;
        }

        knowers.into_iter().collect::<Vec<_>>()
    }
}
