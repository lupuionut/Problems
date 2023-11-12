// 815. Bus Routes
// ---------------

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let mut stations: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut visited: HashSet<i32> = HashSet::new();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

        if source == target {
            return 0;
        }

        routes.iter().enumerate().for_each(|(bus, route)| {
            route.iter().for_each(|&station| {
                stations
                    .entry(station)
                    .and_modify(|mut v| v.push(bus as i32))
                    .or_insert(vec![bus as i32]);
            });
        });

        if let Some(buses) = stations.get(&source) {
            buses.iter().for_each(|&bus| {
                visited.insert(bus);
                for &station in routes[bus as usize].iter() {
                    queue.push_back((station, 1));
                }
            });

            while queue.len() > 0 {
                let (station, cost) = queue.pop_front().unwrap();
                if station == target {
                    return cost;
                }
                if let Some(buses) = stations.get(&station) {
                    buses.iter().for_each(|&bus| {
                        if !visited.contains(&bus) {
                            visited.insert(bus);
                            for &station in routes[bus as usize].iter() {
                                queue.push_back((station, cost + 1));
                            }
                        }
                    });
                }
            }
        }

        -1
    }
}
