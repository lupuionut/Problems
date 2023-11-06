// 1845. Seat Reservation Manager
// ------------------------------

use std::collections::BinaryHeap;

struct SeatManager {
    seats: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        let mut seats = BinaryHeap::new();
        for i in 1..=n {
            seats.push(-i);
        }
        Self { seats }
    }

    fn reserve(&mut self) -> i32 {
        if let Some(seat) = self.seats.pop() {
            return -seat;
        }
        -1
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.seats.push(-seat_number);
    }
}
