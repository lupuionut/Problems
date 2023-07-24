// 731. My Calendar II
// -------------------

use std::collections::BTreeMap;

struct MyCalendarTwo {
    hours: BTreeMap<i32, i32>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        let hours = BTreeMap::new();
        Self { hours }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        // insert current booking
        self.hours.entry(start).and_modify(|v| *v += 1).or_insert(1);
        self.hours.entry(end).and_modify(|v| *v -= 1).or_insert(-1);

        let mut counter = 0;
        for &freq in self.hours.values() {
            counter += freq;
            if counter > 2 {
                // revert current booking
                self.hours.entry(start).and_modify(|v| *v -= 1);
                self.hours.entry(end).and_modify(|v| *v += 1);
                return false;
            }
        }
        return true;
    }
}
