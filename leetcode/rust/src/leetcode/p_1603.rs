/*
    1603. Design Parking System
    ---------------------------
*/
struct ParkingSystem {
    Capacity: (i32, i32, i32),
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            Capacity: (big, medium, small),
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => {
                if self.Capacity.0 > 0 {
                    self.Capacity.0 -= 1;
                    return true;
                }
                false
            }
            2 => {
                if self.Capacity.1 > 0 {
                    self.Capacity.1 -= 1;
                    return true;
                }
                false
            }
            3 => {
                if self.Capacity.2 > 0 {
                    self.Capacity.2 -= 1;
                    return true;
                }
                false
            }
            _ => false,
        }
    }
}
