/*
    1396. Design Underground System
    -------------------------------
*/

use std::collections::HashMap;

struct UndergroundSystem {
    // (from, to) -> (total_time, total_trips)
    average_times: HashMap<(String, String), (u64, u64)>,
    // user_id (start_station, start_time)
    current_travels: HashMap<i32, (String, i32)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        let average_times: HashMap<(String, String), (u64, u64)> = HashMap::new();
        let current_travels: HashMap<i32, (String, i32)> = HashMap::new();
        Self {
            average_times,
            current_travels,
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.current_travels.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some(start) = self.current_travels.get(&id) {
            let start_station = &start.0;
            let time = t - start.1;
            self.average_times
                .entry((start_station.to_string(), station_name))
                .and_modify(|av| {
                    av.1 += 1;
                    av.0 = av.0 + time as u64;
                })
                .or_insert((time as u64, 1));
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some(record) = self.average_times.get(&(start_station, end_station)) {
            return record.0 as f64 / record.1 as f64;
        }
        0.0 as f64
    }
}
