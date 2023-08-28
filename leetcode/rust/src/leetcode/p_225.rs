// 225. Implement Stack using Queues
// ---------------------------------

use std::collections::VecDeque;

struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        return Self {
            queue: VecDeque::new(),
        };
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        let n = self.queue.len() - 1;
        let mut i = 0;

        while i < n {
            if let Some(x) = self.queue.pop_front() {
                self.queue.push_back(x);
            }
            i += 1;
        }
        self.queue.pop_front().unwrap()
    }

    fn top(&mut self) -> i32 {
        let x = self.pop();
        self.queue.push_back(x);
        x
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
