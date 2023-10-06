// 622. Design Circular Queue
// --------------------------

struct MyCircularQueue {
    values: Vec<i32>,
    size: i32,
    front: i32,
    end: i32,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            values: vec![-1; k as usize],
            size: k,
            front: -1,
            end: -1,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if self.is_empty() {
            self.front = 0;
            self.end = 0;
        } else {
            self.end = (self.end + 1) % self.size;
        }

        self.values[self.end as usize] = value;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        if self.front == self.end {
            self.front = -1;
            self.end = -1;
        } else {
            self.front = (self.front + 1) % self.size;
        }
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.values[self.front as usize]
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.values[self.end as usize]
    }

    fn is_empty(&self) -> bool {
        self.front == -1
    }

    fn is_full(&self) -> bool {
        (self.end + 1) % self.size == self.front
    }
}
